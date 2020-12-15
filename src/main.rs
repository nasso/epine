use std::{fs::File, io::Read};

use clap::clap_app;

mod manifest;

use manifest::Manifest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(epine =>
        (version: "1.0")
        (author: "nasso <nassomails@gmail.com>")
        (about: "A Makefile generator for the 21st century")
        (@arg MANIFEST_PATH: --manifest +takes_value
            "Path to the manifest file. By default, Epine will look for
            Epine.toml in the current directory and walk its way up until it
            finds one.")
        (@subcommand create =>
            (about: "Create a new project")
            (version: "1.0")
            (author: "nasso <nassomails@gmail.com>")
            (@group type =>
                (@arg bin: --bin "Create a binary project")
                (@arg lib: --lib "Create a library project")
            )
            (@arg name: +takes_value "Set the executable name. Defaults to the
                directory name.")
            (@arg interactive: -i --interactive "Create the project using the
                interactive prompt")
            (@arg path: +required "The path at which the project should be
                created")
        )
        (@subcommand init =>
            (about: "Initialize a new project")
            (version: "1.0")
            (author: "nasso <nassomails@gmail.com>")
            (@group type =>
                (@arg bin: --bin "Initialize a binary project")
                (@arg lib: --lib "Initialize a library project")
            )
            (@arg name: +takes_value "Set the executable name. Defaults to the
                directory name.")
            (@arg interactive: -i --interactive "Initialize the project using
                the interactive prompt")
            (@arg path: +required "The path at which the project should be
                initialized")
        )
    )
    .get_matches();

    // get manifest path
    let manifest_path = matches.value_of("MANIFEST_PATH").unwrap_or("Epine.toml");
    let mut manifest_file = File::open(manifest_path)?;
    let mut manifest_source = String::new();

    // read the source
    manifest_file.read_to_string(&mut manifest_source)?;

    let manifest: Manifest = toml::from_str(&manifest_source)?;

    println!("{:#?}", manifest);

    Ok(())
}
