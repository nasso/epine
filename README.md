# Epine
[![Crates.io](https://img.shields.io/crates/v/epine.svg)](https://crates.io/crates/epine)

Epine is a powerful Makefile generator using the Lua programming language for
its configuration.

## Goals

- Generating a single Makefile
- Scaling up as well as scaling down
- Allowing anyone to share their Epine modules and helper libraries

## Non-goals

- Replacing `make`
- Replacing a package manager

## Installation

Install the latest release of Epine with Cargo:

```
cargo install epine
```

If you don't have Cargo/Rust installed, you can get it on
[rustup.rs](https://rustup.rs).

## Hello, Epine!

Go in the folder of your choice, where you want your Makefile to be generated.

```
mkdir hello-epine
cd hello-epine
```

In this new folder, create a new file named `Epine.lua`:

```lua
return {
    action "hello" {
        echo("hello!");
    };
}
```

To (re)generate the Makefile, simply run `epine`, without any argument.

```
epine
```

Epine will run the Lua code contained in the `Epine.lua` file and generate a
Makefile according to what this file returns to it.

```Makefile
hello:
    @echo hello!
.PHONY: hello
```

## Using remote modules

The way the generated Makefile is described is very close to what ends up being
generated. If it was just that, Epine would just be a very annoying way to write
your Makefiles... in Lua.

This is why Epine has the ability to download and load Lua modules from GitHub
repositories. This makes it really easy to reuse and share helper functions and
libraries. For now, only GitHub is supported, but I would love to see Epine
support many more sources in the future; contributions are welcome!

Here's an example showing how simple the generation of a Makefile can become:

```lua
-- the `tek` module, which was made for students at Epitech, takes care of
-- generating many rules automatically, like "all", "clean", "fclean", etc...
-- it also generates the proper rules to build and run unit tests!
local tek = require "@nasso/epine-tek/v0.1.0-alpha"

-- project metadata (its name, and the targets built by the "all" target)
tek:project "libmy" {"libmy.a", "hello"}

-- a C static library!
tek:static "libmy.a" {
    language = "C";
}

-- a C binary using the library!
tek:binary "hello" {
    language = "C";
    prerequisites = {"libmy.a"};
    srcs = {"main.c"};
    libs = {"my"};
}

-- return the Makefile description to Epine for generation
return tek:make()
```

This simple script, which is only 12 lines of code, generates a clean,
human-readable, and *fully functional*
[Makefile](examples/github-fetch/Makefile)! Of course, this is only a mere
example of what Epine is capable of.

Because it's all Lua anyway, you can imagine any sort of API to describe your
project, and you can share it with anyone!

## Status

Epine is still a *very* young project. The main reason I built it was because I
didn't want to copy-paste my Makefiles between the different school projects I
had to build during my first years as an [Epitech] student.

[Epitech]: https://epitech.eu

## License

Epine is licensed under the terms of both the MIT license and the Apache License
(Version 2.0), at your choice.

See LICENSE-MIT and LICENSE-APACHE-2.0 files for the full texts.
