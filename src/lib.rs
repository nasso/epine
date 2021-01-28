use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use mlua::{Lua, LuaSerdeExt, StdLib};
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error {0}")]
    Io(#[from] io::Error),
    #[error("lua error {0}")]
    Lua(#[from] mlua::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Vardef {
    Recursive { name: String, value: String },
    Simple { name: String, value: String },
    Conditional { name: String, value: String },
    Shell { name: String, value: String },
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
            let mut path = path.join(&name);
            path.set_extension("lua");

            let mut source = String::new();

            Ok(|| -> Result<mlua::Function> {
                File::open(&path)?.read_to_string(&mut source)?;

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

impl Makefile {
    pub fn from_lua_source(src: &str, name: &str) -> Result<Self> {
        let lua = Lua::new_with(
            StdLib::TABLE
                | StdLib::MATH
                | StdLib::OS
                | StdLib::STRING
                | StdLib::PACKAGE
        )?;

        // epine will look for modules in the following locations, in order:

        // 1. current working directory (".")
        // 2. ./.epine/modules
        if let Ok(cwd) = std::env::current_dir() {
            add_require_search_path(&lua, cwd.clone())?;
            add_require_search_path(&lua, cwd.join(".epine").join("modules"))?;
        }

        // 3. <epine_binary>/modules
        if let Ok(epine) = std::env::current_exe() {
            add_require_search_path(&lua, epine.join("modules"))?;
        }

        // 4. ~/.local/share/epine/modules
        if let Some(dirs) = ProjectDirs::from("", "", "epine") {
            add_require_search_path(&lua, dirs.data_dir().join("modules"))?;
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
                writeln!(f, "{} = {}", name, value)
            }
            MakefileThing::Vardef(Vardef::Simple { name, value }) => {
                writeln!(f, "{} := {}", name, value)
            }
            MakefileThing::Vardef(Vardef::Conditional { name, value }) => {
                writeln!(f, "{} ?= {}", name, value)
            }
            MakefileThing::Vardef(Vardef::Shell { name, value }) => {
                writeln!(f, "{} != {}", name, value)
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
