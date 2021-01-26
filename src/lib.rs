use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use mlua::{Lua, StdLib};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error {0}")]
    Io(#[from] io::Error),
    #[error("lua error {0}")]
    Lua(#[from] mlua::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum MakefileThing {}

#[derive(Debug)]
pub struct Makefile {
    pub things: Vec<MakefileThing>,
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
            StdLib::TABLE | StdLib::MATH | StdLib::OS | StdLib::STRING | StdLib::PACKAGE,
        )?;

        // epine will look for modules in the following locations, in order:

        // 1. current working directory (".")
        // 2. ./modules
        if let Ok(cwd) = std::env::current_dir() {
            add_require_search_path(&lua, cwd.clone())?;
            add_require_search_path(&lua, cwd.join("modules"))?;
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

        let r = lua.load(src).set_name(name)?.call::<_, mlua::Table>(())?;

        println!("{} things", r.len()?);

        Ok(Self { things: vec![] })
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
