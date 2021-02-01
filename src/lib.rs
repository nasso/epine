use std::{
    fmt::{Display, Formatter},
    fs::{create_dir_all, rename, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use io::ErrorKind;
use mlua::{Lua, LuaSerdeExt, StdLib};
use serde::Deserialize;
use tar::{Archive, EntryType};
use tokio::runtime;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error {0}")]
    Io(#[from] io::Error),
    #[error("lua error {0}")]
    Lua(#[from] mlua::Error),
    #[error("io error {0}")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Vardef {
    Recursive { name: String, value: String },
    Simple { name: String, value: String },
    Conditional { name: String, value: String },
    Shell { name: String, value: String },
    Append { name: String, value: String },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Directive {}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum MakefileThing {
    Comment(String),
    Vardef(Vardef),
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

fn _get_things<'a, T: Deserialize<'a>>(lua: &'a Lua, t: mlua::Table<'a>) -> Result<Vec<T>> {
    let mut things = Vec::new();

    for v in t.sequence_values::<mlua::Value>() {
        match v? {
            mlua::Value::Table(v) if v.len()? != 0 => {
                things.append(&mut _get_things(lua, v)?);
            }
            mlua::Value::Table(v)
                if v.clone()
                    .pairs::<mlua::Value, mlua::Value>()
                    .next()
                    .is_none() =>
            {
                ()
            }
            v => things.push(lua.from_value(v)?),
        };
    }

    Ok(things)
}

fn add_require_search_path(lua: &Lua, path: PathBuf) -> Result<()> {
    let searchers = lua
        .globals()
        .get::<_, mlua::Table>("package")?
        .get::<_, mlua::Table>("searchers")?;

    searchers.set(
        searchers.len()? + 1,
        lua.create_function::<String, Option<mlua::Function>, _>(move |lua, name: String| {
            let path = path.join(&name);

            let mut source = String::new();

            Ok(|| -> Result<mlua::Function> {
                File::open(path.with_extension("lua"))
                    .or_else(|_| File::open(path.join("init.lua")))?
                    .read_to_string(&mut source)?;

                Ok(lua.create_function(move |lua, p: mlua::MultiValue| {
                    Ok(lua
                        .load(&source)
                        .set_name(&name)?
                        .call::<_, mlua::MultiValue>(p)?)
                })?)
            }()
            .ok())
        })?,
    )?;

    Ok(())
}

fn add_require_github_importer(lua: &Lua, ghfolder: PathBuf) -> Result<()> {
    let searchers = lua
        .globals()
        .get::<_, mlua::Table>("package")?
        .get::<_, mlua::Table>("searchers")?;

    searchers.set(
        searchers.len()? + 1,
        lua.create_function::<String, Option<mlua::Function>, _>(move |lua, name: String| {
            if name.starts_with("@") {
                let (org, repo, tag) = {
                    let repo: Vec<_> = name[1..].split('/').collect();

                    if repo.len() != 3 {
                        return Ok(None);
                    }

                    (repo[0], repo[1], repo[2])
                };

                let mut source = String::new();
                Ok(|| -> Result<mlua::Function> {
                    let target = format!("https://github.com/{}/{}/tarball/{}", org, repo, tag);
                    let tmp_dir = tempfile::Builder::new().prefix("epine").tempdir()?;
                    let rt = runtime::Builder::new_current_thread()
                        .enable_io()
                        .build()
                        .unwrap();
                    println!("attempting download of: {}", target);
                    let response = rt.block_on(reqwest::get(&target))?;
                    println!("res: {}", response.status());

                    let dlpath = {
                        let fname = format!("{}.{}.{}.tar.gz", org, repo, tag);

                        let fname = tmp_dir.path().join(fname);
                        fname
                    };

                    // download
                    {
                        let mut dest = File::create(&dlpath)?;
                        let content = rt.block_on(response.bytes())?;
                        std::io::copy(&mut content.as_ref(), &mut dest)?;
                    }

                    // extract
                    {
                        let tar_gz = File::open(&dlpath)?;
                        let tar = GzDecoder::new(tar_gz);
                        let mut archive = Archive::new(tar);
                        archive.unpack(&ghfolder)?;
                    }

                    // rename
                    let root = {
                        let tar_gz = File::open(&dlpath)?;
                        let rootname = {
                            let tar = GzDecoder::new(tar_gz);
                            let mut archive = Archive::new(tar);
                            let root = archive
                                .entries()?
                                .find(|file| match file {
                                    Ok(file) => file.header().entry_type() == EntryType::Directory,
                                    _ => false,
                                })
                                .ok_or(std::io::Error::new(
                                    ErrorKind::NotFound,
                                    "empty archive",
                                ))??;
                            root.path()?.into_owned()
                        };

                        let dest = ghfolder.join(format!("@{}", org)).join(repo);
                        let root = dest.join(tag);
                        create_dir_all(&dest)?;
                        rename(ghfolder.join(rootname), &root)?;
                        root
                    };

                    File::open(root.join("init.lua"))?.read_to_string(&mut source)?;

                    let name = name.clone();
                    Ok(lua.create_function(move |lua, p: mlua::MultiValue| {
                        Ok(lua
                            .load(&source)
                            .set_name(&name)?
                            .call::<_, mlua::MultiValue>(p)?)
                    })?)
                }()
                .map_err(|e| {
                    println!("ERROR: {:?}", e);
                    e
                })
                .ok())
            } else {
                Ok(None)
            }
        })?,
    )?;

    Ok(())
}

impl Makefile {
    pub fn from_lua_source(src: &str, name: &str) -> Result<Self> {
        let lua = Lua::new_with(
            StdLib::TABLE | StdLib::MATH | StdLib::OS | StdLib::STRING | StdLib::PACKAGE,
        )?;

        // epine will look for modules in the following locations, in order:

        // 1. current working directory (".")
        // 2. ./.epine/github
        if let Ok(cwd) = std::env::current_dir() {
            add_require_search_path(&lua, cwd.clone())?;
            add_require_search_path(&lua, cwd.join(".epine").join("github"))?;
            add_require_github_importer(&lua, cwd.join(".epine").join("github"))?;
        }

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

        Makefile::from_lua_source(&source[..], &path.as_ref().to_string_lossy())
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
            MakefileThing::Vardef(Vardef::Recursive { name, value }) => {
                if value == "" {
                    writeln!(f, "{} =", name)
                } else {
                    writeln!(f, "{} = {}", name, value)
                }
            }
            MakefileThing::Vardef(Vardef::Simple { name, value }) => {
                if value == "" {
                    writeln!(f, "{} :=", name)
                } else {
                    writeln!(f, "{} := {}", name, value)
                }
            }
            MakefileThing::Vardef(Vardef::Conditional { name, value }) => {
                if value == "" {
                    writeln!(f, "{} ?=", name)
                } else {
                    writeln!(f, "{} ?= {}", name, value)
                }
            }
            MakefileThing::Vardef(Vardef::Shell { name, value }) => {
                if value == "" {
                    writeln!(f, "{} !=", name)
                } else {
                    writeln!(f, "{} != {}", name, value)
                }
            }
            MakefileThing::Vardef(Vardef::Append { name, value }) => {
                if value == "" {
                    writeln!(f, "{} +=", name)
                } else {
                    writeln!(f, "{} += {}", name, value)
                }
            }
            MakefileThing::Directive(_) => unimplemented!(),
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
