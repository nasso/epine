use clap::clap_app;

use epine::Makefile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(epine =>
        (version: "1.0")
        (author: "nasso <nassomails@gmail.com>")
        (about: "A Makefile generator for the 21st century")
        (@arg EPINE_FILE: -f --file +takes_value
            "Path to the Epine file. By default, Epine will look for Epine.lua in the current directory and walk its way up until it finds one.")
    )
    .get_matches();

    let path = matches.value_of("EPINE_FILE").unwrap_or("Epine.lua");

    // get config file
    let makefile = match Makefile::from_lua_file(&path) {
        Ok(makefile) => makefile,
        Err(epine::Error::Lua(lua_error)) => {
            eprintln!("{}", lua_error);
            std::process::exit(84)
        }
        Err(epine::Error::Io(e)) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                eprintln!("{}: file not found", path);
                std::process::exit(1)
            } else {
                return Err(e.into())
            }
        }
    };

    match makefile.generate() {
        Ok(source) => println!("{}", source),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
