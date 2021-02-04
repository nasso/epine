use std::{
    fmt::{Display, Formatter},
    fs::{create_dir_all, rename, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use flate2::read::GzDecoder;
use io::ErrorKind;
use mlua::{Lua, LuaSerdeExt, StdLib};
use serde::Deserialize;
use tar::{Archive, EntryType};
use tokio::runtime;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Lua(#[from] mlua::Error),
    #[error("{0}")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize)]
pub enum VarFlavor {
    Recursive,
    Simple,
    Conditional,
    Shell,
    Append,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Directive {
    Include(Option<Vec<String>>),
    SInclude(Option<Vec<String>>),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum MakefileThing {
    Comment(String),
    Vardef {
        name: String,
        value: String,
        flavor: VarFlavor,
        targets: Option<Vec<String>>,
    },
    Directive(Directive),
    Break,
    ExplicitRule {
        targets: Vec<String>,
        prerequisites: Option<Vec<String>>,
        recipe: Option<Vec<String>>,
    },
    PatternRule {
        patterns: Vec<String>,
        prerequisites: Option<Vec<String>>,
        recipe: Option<Vec<String>>,
    },
    StaticPatternRule {
        targets: Vec<String>,
        target_pattern: String,
        prereq_patterns: Option<Vec<String>>,
        recipe: Option<Vec<String>>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Makefile {
    things: Vec<MakefileThing>,
}

fn local_require_searcher<'lua>(
    lua: &'lua Lua,
    path: &Path,
    name: String,
) -> Result<(mlua::Function<'lua>, String)> {
    let path = path.join(&name);

    let mut source = String::new();

    let mut actualpath = path.with_extension("lua");

    match File::open(&actualpath) {
        Ok(file) => Ok(file),
        Err(_) => {
            actualpath = path.join("init.lua");
            File::open(&actualpath)
        }
    }?
    .read_to_string(&mut source)?;

    let parent = actualpath
        .canonicalize()?
        .parent()
        .map_or(PathBuf::default(), |p| PathBuf::from(p));
    let actualpath = actualpath.to_str().unwrap_or(&name).to_owned();
    Ok((
        lua.create_function(move |lua, p: mlua::MultiValue| {
            let searchers = lua
                .globals()
                .get::<_, mlua::Table>("package")?
                .get::<_, mlua::Table>("searchers")?;
            let old_dir_searcher = searchers.get::<_, mlua::Value>(1)?;

            searchers.set(1, add_require_search_path(lua, parent.clone())?)?;

            let module = lua
                .load(&source)
                .set_name(&name)?
                .call::<_, mlua::MultiValue>(p)?;

            searchers.set(1, old_dir_searcher)?;

            Ok(module)
        })?,
        actualpath,
    ))
}

fn add_require_search_path<'a>(lua: &'a Lua, path: PathBuf) -> mlua::Result<mlua::Function<'a>> {
    lua.create_function(
        move |lua, name| match local_require_searcher(lua, &path, name) {
            Ok((f, n)) => Ok((Some(f), n)),
            Err(e) => Ok((None, format!("couldn't load the module: {}", e))),
        },
    )
}

fn try_module_download(path: &Path, ident: &str) -> Option<PathBuf> {
    let (org, repo, tag) = {
        let repo: Vec<_> = ident.split('/').collect();

        if repo.len() != 3 {
            return None;
        }

        (repo[0], repo[1], repo[2])
    };

    let target = format!("https://github.com/{}/{}/tarball/{}", org, repo, tag);
    let tmp_dir = tempfile::Builder::new().prefix("epine").tempdir().ok()?;
    let rt = runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    print!("getting {}/{} ({})... ", org, repo, tag);
    std::io::stdout().flush().ok()?;
    let response = rt.block_on(reqwest::get(&target)).ok()?;
    println!("{}", response.status());

    let dlpath = {
        let fname = format!("{}.{}.{}.tar.gz", org, repo, tag);

        let fname = tmp_dir.path().join(fname);
        fname
    };

    // download
    {
        let mut dest = File::create(&dlpath).ok()?;
        let content = rt.block_on(response.bytes()).ok()?;
        std::io::copy(&mut content.as_ref(), &mut dest).ok()?;
    }

    // extract
    {
        let tar_gz = File::open(&dlpath).ok()?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(&path).ok()?;
    }

    // rename (the archive has a single folder inside with some random name)
    // there is probably a better way to do this
    let root = {
        let tar_gz = File::open(&dlpath).ok()?;

        // find the name of the root folder
        let archive_root_name = {
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            let root = archive
                .entries()
                .ok()?
                .find(|file| match file {
                    Ok(file) => file.header().entry_type() == EntryType::Directory,
                    _ => false,
                })
                .ok_or(std::io::Error::new(ErrorKind::NotFound, "empty archive"))
                .ok()?
                .ok()?;
            root.path().ok()?.into_owned()
        };

        let dest = path.join(format!("@{}", org)).join(repo);
        create_dir_all(&dest).ok()?;
        let dest_module_root = dest.join(tag);
        rename(path.join(archive_root_name), &dest_module_root).ok()?;
        dest_module_root
    };

    Some(root)
}

fn add_require_github_importer(
    lua: &Lua,
    searchers: &mlua::Table,
    ghfolder: PathBuf,
) -> Result<()> {
    searchers.set(
        searchers.len()? + 1,
        lua.create_function(move |lua, name: String| {
            if !name.starts_with("@") {
                return Ok((None, String::from("not a remote module")));
            }

            if let Some(path) = try_module_download(&ghfolder, &name[1..]) {
                match local_require_searcher(lua, &path, String::from("init")) {
                    Ok((f, n)) => Ok((Some(f), n)),
                    Err(e) => Ok((None, format!("couldn't load the module: {}", e))),
                }
            } else {
                return Ok((None, String::from("couldn't fetch remote module")));
            }
        })?,
    )?;

    Ok(())
}

impl Makefile {
    pub fn from_lua_source(src: &str, name: &str, dir: Option<&Path>) -> Result<Self> {
        let lua = Lua::new_with(
            StdLib::TABLE | StdLib::MATH | StdLib::OS | StdLib::STRING | StdLib::PACKAGE,
        )?;

        let package = lua.globals().get::<_, mlua::Table>("package")?;
        let searchers = lua.create_table()?;

        // if a "working directory" is specified (usually the folder in which Epine.lua is located)
        // epine will look for modules in the current working directory
        if let Some(dir) = dir {
            searchers.set(
                searchers.len()? + 1,
                add_require_search_path(&lua, dir.to_owned())?,
            )?;
        }

        // github modules are downloaded in a global directory to avoid downloading them many times
        // also fixes issues where the module contains examples that can get caught by a "find"
        if let Some(proj_dirs) = ProjectDirs::from("", "", "epine") {
            let dir = proj_dirs.cache_dir();

            // searcher that finds previously downloaded modules
            searchers.set(
                searchers.len()? + 1,
                add_require_search_path(&lua, dir.join("github"))?,
            )?;
            // the importer is the one that downloads new modules
            add_require_github_importer(&lua, &searchers, dir.join("github"))?;
        }

        package.set("searchers", searchers)?;

        lua.load(include_str!("./api.lua"))
            .set_name("api")?
            .exec()?;

        let makefile_def = lua.load(src).set_name(name)?.call(())?;

        let makefile_def = lua
            .load(include_str!("./normalize.lua"))
            .set_name("normalize")?
            .eval::<mlua::Function>()?
            .call::<mlua::Value, _>(makefile_def)?;

        Ok(Self {
            things: lua.from_value(makefile_def)?,
        })
    }

    pub fn from_lua_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(&path)?;
        let mut source = String::new();

        file.read_to_string(&mut source)?;

        Makefile::from_lua_source(
            &source[..],
            &path.as_ref().to_string_lossy(),
            path.as_ref().parent(),
        )
    }

    pub fn generate(&self) -> Result<String> {
        Ok(String::new())
    }
}

impl Display for Makefile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for thing in self.things.iter() {
            write!(f, "{}", thing)?;
        }

        Ok(())
    }
}

impl Display for MakefileThing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MakefileThing::Comment(line) => writeln!(f, "#{}", line),
            MakefileThing::Vardef {
                name,
                value,
                flavor,
                targets,
            } => {
                if let Some(targets) = targets {
                    write!(f, "{}: ", targets.join(" "))?;
                }

                write!(f, "{} ", name)?;

                match flavor {
                    VarFlavor::Recursive => write!(f, "=")?,
                    VarFlavor::Simple => write!(f, ":=")?,
                    VarFlavor::Conditional => write!(f, "?=")?,
                    VarFlavor::Shell => write!(f, "!=")?,
                    VarFlavor::Append => write!(f, "+=")?,
                }

                if value == "" {
                    writeln!(f)
                } else {
                    writeln!(f, " {}", value)
                }
            }
            MakefileThing::Directive(Directive::Include(fnames)) => {
                write!(f, "include")?;

                if let Some(fnames) = fnames {
                    for fname in fnames.iter() {
                        write!(f, " {}", fname)?;
                    }
                }

                writeln!(f)
            }
            MakefileThing::Directive(Directive::SInclude(fnames)) => {
                write!(f, "-include")?;

                if let Some(fnames) = fnames {
                    for fname in fnames.iter() {
                        write!(f, " {}", fname)?;
                    }
                }

                writeln!(f)
            }
            MakefileThing::Break => {
                writeln!(f)
            }
            MakefileThing::ExplicitRule {
                targets,
                prerequisites,
                recipe,
            } => {
                write!(f, "{}:", targets.join(" "))?;

                if let Some(prereqs) = prerequisites {
                    for pre in prereqs.iter() {
                        write!(f, " {}", pre)?;
                    }
                }

                writeln!(f)?;

                if let Some(steps) = recipe {
                    for step in steps {
                        writeln!(f, "\t{}", step)?;
                    }
                }

                Ok(())
            }
            MakefileThing::PatternRule {
                patterns,
                prerequisites,
                recipe,
            } => {
                write!(f, "{}:", patterns.join(" "))?;

                if let Some(prereqs) = prerequisites {
                    for pre in prereqs.iter() {
                        write!(f, " {}", pre)?;
                    }
                }

                writeln!(f)?;

                if let Some(steps) = recipe {
                    for step in steps {
                        writeln!(f, "\t{}", step)?;
                    }
                }

                Ok(())
            }
            MakefileThing::StaticPatternRule {
                targets,
                target_pattern,
                prereq_patterns,
                recipe,
            } => {
                write!(f, "{}: {}", targets.join(" "), target_pattern)?;

                if let Some(prereq_pats) = prereq_patterns {
                    write!(f, ":")?;

                    for pp in prereq_pats.iter() {
                        write!(f, " {}", pp)?;
                    }
                }

                writeln!(f)?;

                if let Some(steps) = recipe {
                    for step in steps {
                        writeln!(f, "\t{}", step)?;
                    }
                }

                Ok(())
            }
        }
    }
}
