# Epine

Epine is a tiny Makefile generator for C/C++ projects. Although it was primarily
built for [Epitech] students, we aim to make it available for anyone needing a
quick tool to generate Makefiles for their own projects.

[Epitech]: https://epitech.eu

## Goals

- Generating a standalone Makefile
- Make the process of creating new C/C++ projects quick and easy
- Encourage code reuse
- Make it easy to maintain and use your own libraries
- **Make it usable by any Epitech student without them being penalized :)**

## Non-goals

- Replace your build process
- Providing a centralized package registry
- Be used as a large scale package manager
- Replacing Makefiles
- Replacing git

## Quickstart

```
$ epine create -i
Project name: corewar
Template:
0) Empty
1) C project
Your choice: 1

Successfully created project 'corewar' in `corewar`.
$ tree corewar
corewar
├── include
│   └── main.h
├── src
│   └── main.c
├── tests
│   └── it_works.c
├── .gitignore
├── Epine.toml
└── Makefile
```

### Regenerating the Makefile

To regenerate your Makefile (after modifying the manifest), simply run `epine`
within your project directory:
```sh
$ cd corewar
$ epine
< Epine.toml
> Makefile
```

## Libraries

If you want to link to a particular library, simply add it to your target's
`cflags`:

```toml
cflags = ["-lcsfml-system", "-lcsfml-graphics"]
```

## Dependencies

Libraries in C have always been annoying when compared to more modern solutions,
like [npm](https://www.npmjs.com/), [Cargo](https://crates.io/) and the like.
Epine aims to make it really easy to add library dependencies to your project,
by taking inspiration from these established solutions.

Say you have made a library and have it on your personal GitHub account.
All you have to do to add a dependency is add this to your manifest:

```toml
dependencies = ["https://github.com/yourname/libmy"]
```

### What's the difference between a library and dependency?

A library is manually linked with `cflags` as shown above. You also have to
manually handle include paths to make sure you can `#include` its headers in
your source. If the library code is updated, it must be manually rebuilt too.

On the other hand, a dependency, is transparently managed. All you have to do is
add an entry to your manifest, and Epine will automatically add the required
compiler flags and include paths. The generated Makefile will also call `make`
in the library's source directory, to keep it up-to-date with the latest changes
if you make any.

You're probably wondering how that works. Following is a rough explanation.

### Compiler flags

The name of the library is inferred from the name of the remote repository, the
name of the directory or the name of the target.

#### Examples

- From a target name: `"libcorewar.a" -> -lcorewar`
- From a directory name: `"./libmy" -> -lmy`
- From a repository URI: `"https://github.com/yourname/libjzon" -> -ljzon`
- Explicitly specified: `{ path = "./jsonlib", name = "libjzon" } -> -ljzon`

### Include paths

To know what include paths to add for each dependency, Epine will try the
following, in order:

- If an Epine manifest is found in the dependency root directory, include paths
    are read from it.
- If it exists, add the `include/` folder.
- If specified, add the paths specified in the `include` field of the dependency
    entry.
    ```toml
    dependencies = [
        {
            path = "./libjson",
            include = ["./libjson/headers"]
        }
    ]
    ```

## CLI reference

### `epine create [options] <path>`

Create a new project at the given path. This command fails if `path` already
exists. To initialize a project in a pre-existing directory, use `epine init`.

#### Arguments

`path` The path at which the project should be created.

#### Options

`--bin` Create a binary project (`corewar` executable). This is the default
    behaviour.

`--lib` Create a library project (`libcorewar.a`).

`--name` _name_ Set the executable name. Defaults to the directory name.

`-i`, `--interactive` Create the project using the interactive prompt.

#### Examples

##### To create a new binary project
```sh
epine create corewar
```

##### To create a new library project
```sh
epine create --lib libmy
```

### `epine init [options] [path]`

Initialize a project at the given path. This command fails if `path` doesn't
exist. To create a new project in a new directory, use `epine create`.

#### Arguments

`path` The directory in which to initialize the project. Defaults to the current
    directory (`.`).

#### Options

`--bin` Initialize a binary package. This is the default behaviour.

`--lib` Initialize a library package.

`--name` _name_ Set the executable name. Defaults to the directory name.

`-i`, `--interactive` Create the project using the interactive prompt.

#### Examples

##### To create a new binary project
```sh
epine create corewar
```

##### To create a new library project
```sh
epine create --lib libmy
```

## Manifest reference (`Epine.toml`)

```toml
# Optionally specify the default target (the one built when running `make`)
default-target = "corewar"

# Where remote dependencies are downloaded (default = "epine_dependencies")
dependencies = "epine_modules"

# You can add a binary target (where object files are linked)
[[bin]]
# Name of the binary
name = "corewar"

# Source files
src = ["./src/**/*.c"]

# Include directories
include = ["./include"]

# Override the default compiler (default = "cc")
cc = "gcc"

# You can add compiler flags
lflags = ["-lcsfml-system", "-lcsfml-graphics", "-lcsfml-audio"]
cflags = ["-Wall", "-Wextra"]

# You can specify commands to be run before and after each build
# If a command fails, the build is terminated
run-before = ["astyle --style=allman --recursive ./src/*.c ./include/*.h"]
run-after = ["notify-send '$(EPINE_NAME)' 'Build ready.'"]

# Feature flags are inherited by all dependencies
features = ["calloc", "malloc", "free", "open", "read", "write", "close"]
# This defines -DEPINE_FEAT_CALLOC, -DEPINE_FEAT_MALLOC, ...

# Dependencies are libraries that are built before the target. Remote
# dependencies are downloaded when you run `epine` or `epine refetch`
dependencies = [
    # Another target from the same project, defined elsewhere in the manifest
    { target = "libcorewar.a" },

    # Local dependency
    { path = "./libmy" },

    # Remote Git dependency
    { git = "https://github.com/yourname/libjson" },

    # All of these can be shortened to just:
    "libcorewar.a",
    "./libmy",
    "https://github.com/yourname/libjson",

    # Complete syntax
    {
        path = "./libmy",

        # Epine will run `make libmy.a` to build the dependency
        target = "libmy.a",

        # The name is inferred in most cases, but you can explicitely specify it
        name = "libmy",
    },
]

# You can also add a library target (object files are archived with `ar -c`)
[[lib]]
name = "libcorewar.a"
src = ["./libcorewar/src/**/*.c"]
include = ["./libcorewar/include"]
cflags = ["-Wall", "-Wextra"]
dependencies = [{ git = "https://github.com/yourname/libmy" }]

# You can add as many additional targets as you want, but names must be unique.
[[bin]]
name = "asm"
src = ["./asm/src/**/*.c"]
include = ["./asm/include"]
dependencies = [{ git = "https://github.com/yourname/libmy" }]

# An example target for unit testing using Criterion
[[bin]]
name = "unit_tests"
src = ["./tests/**/*.c"]
lflags = ["-lcriterion"]
# Equivalent to [{ path = ".", name = "libcorewar.a" }]: depend on the library
# defined above for testing
dependencies = ["libcorewar.a"]
```
