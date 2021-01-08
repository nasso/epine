use std::sync::{Arc, Mutex};

use mlua::StdLib;
use mlua::{prelude::*, LuaSerdeExt};

use crate::Configuration;

pub fn process_config(config: Configuration, source: &str) -> Result<Configuration, mlua::Error> {
    let config = Arc::new(Mutex::new(config));

    {
        let lua = Lua::new_with(
            StdLib::TABLE | StdLib::MATH | StdLib::OS | StdLib::STRING | StdLib::PACKAGE,
        )?;

        let api = lua.create_table()?;

        // begin_target(name)
        {
            let config = Arc::clone(&config);

            api.set(
                "add_target",
                lua.create_function_mut(move |lua, target: LuaValue| {
                    let target = lua.from_value(target)?;
                    let mut config = config.lock().unwrap();

                    config.targets.push(target);

                    Ok(())
                })?,
            )?;
        }

        lua.globals().set("epine", api)?;

        let chunk = lua.load(include_str!("./api.lua"));
        chunk.exec()?;

        let chunk = lua.load(source);
        chunk.exec()?;

        lua.load("if epine.on_end then epine.on_end() end").exec()?;
    }

    Ok(Arc::try_unwrap(config).unwrap().into_inner().unwrap())
}
