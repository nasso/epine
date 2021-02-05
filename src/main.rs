use std::{fs::File, io::Write, path::PathBuf};

use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};

use epine::Makefile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(epine =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg EPINE_FILE: -f --file +takes_value "Path to the Epine file. By default, Epine will look for Epine.lua in the current directory and walk its way up until it finds one.")
        (@arg OUTPUT_FILE: -o --output +takes_value "Path to the Makefile to be generated. Defaults to \"Makefile\" in the current directory.")
        (@arg ARGS: +last +multiple)
    )
    .get_matches();

    let path: PathBuf = matches.value_of("EPINE_FILE").unwrap_or("Epine.lua").into();
    let dest: PathBuf = matches
        .value_of("OUTPUT_FILE")
        .map(PathBuf::from)
        .unwrap_or(path.with_file_name("Makefile"));
    let args = matches
        .values_of("ARGS")
        .map(|vals| vals.collect())
        .unwrap_or(Vec::new());

    // get config file
    let makefile = match Makefile::from_lua_file(&path, &args) {
        Ok(makefile) => makefile,
        Err(epine::Error::Io(e)) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                eprintln!("{:?}: file not found", path);
                std::process::exit(1)
            } else {
                return Err(e.into());
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(84)
        }
    };

    let mut output = File::create(dest)?;
    write!(output, "{}", makefile)?;

    Ok(())
}
