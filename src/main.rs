use clap::clap_app;

mod manifest;

fn main() {
    let _matches = clap_app!(epine =>
        (version: "1.0")
        (author: "nasso <nassomails@gmail.com>")
        (about: "A Makefile generator for the 21st century")
        (@arg MANIFEST_PATH: --manifest-path +takes_value
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
}
