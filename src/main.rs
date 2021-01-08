use std::{convert::TryFrom, fs::File, io::Read};

use clap::clap_app;

use epine::Configuration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(epine =>
        (version: "1.0")
        (author: "nasso <nassomails@gmail.com>")
        (about: "A Makefile generator for the 21st century")
        (@arg CONFIG_FILE: --config +takes_value
            "Path to the configuration file. By default, Epine will look for Epine.toml in the current directory and walk its way up until it finds one.")
        (@subcommand create =>
            (about: "Create a new project")
            (version: "1.0")
            (author: "nasso <nassomails@gmail.com>")
            (@group type =>
                (@arg bin: --bin "Create a binary project")
                (@arg lib: --lib "Create a library project")
            )
            (@arg name: +takes_value "Set the executable name. Defaults to the directory name.")
            (@arg interactive: -i --interactive "Create the project using the interactive prompt")
            (@arg path: +required "The path at which the project should be created")
        )
        (@subcommand init =>
            (about: "Initialize a new project")
            (version: "1.0")
            (author: "nasso <nassomails@gmail.com>")
            (@group type =>
                (@arg bin: --bin "Initialize a binary project")
                (@arg lib: --lib "Initialize a library project")
            )
            (@arg name: +takes_value "Set the executable name. Defaults to the directory name.")
            (@arg interactive: -i --interactive "Initialize the project using the interactive prompt")
            (@arg path: +required "The path at which the project should be initialized")
        )
    )
    .get_matches();

    let mut file = File::open(matches.value_of("CONFIG_FILE").unwrap_or("Epine.lua"))?;
    let mut source = String::new();

    file.read_to_string(&mut source)?;

    // get config file
    let config = match Configuration::try_from(&source[..]) {
        Ok(config) => config,
        Err(epine::Error::Lua(lua_error)) => {
            eprintln!("{}", lua_error);
            std::process::exit(84)
        }
        Err(e) => return Err(e.into()),
    };

    match config.generate() {
        Ok(source) => println!("{}", source),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
