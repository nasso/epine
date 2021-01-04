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

Successfully created project 'corewar' in 'corewar'.
$ tree corewar
corewar
├── include
│   └── main.h
├── src
│   └── main.c
├── tests
│   └── it_works.c
├── .gitignore
└── Epine.lua
```

```lua
target "corewar"
    files { match "./src/*.c" }
    include "./include"
```

### Regenerating the Makefile

To regenerate your Makefile (after modifying the config file), simply run
`epine` within your project directory:
```sh
$ cd corewar
$ epine
< Epine.lua
> Makefile
```

## Libraries

If you want to link to a particular system library:

```lua
target "corewar"
    files { match "./src/*.c" }
    include "./include"
    link "csfml-system"
    link "csfml-graphics" from "./lib/csfml"
```

## Dependencies

Sometimes linking to a library isn't enough. For example you may want to build
it from source. In this case, you probably want to use a *dependency* instead:

```lua
target "corewar"
    files { match "./src/*.c" }
    include "./include"
    use "libmy.a" from "./lib/libmy"
```

You can also pull it from a remote Git repository:

```lua
target "corewar"
    files { match "./src/*.c" }
    include "./include"
    use "my" from "https://github.com/nasso/libmy.git"
```

### Include paths

To know what include paths to add for each dependency, Epine will try the
following, in order:

- If an Epine config file is found in the dependency's root directory, include
    paths are read from it.
- Otherwise, add the `include/` folder if it exists.
- Otherwise, add the paths specified in the `headers` field of the dependency
    entry, if specified.
    ```lua
    use {
        name = "list";
        headers = { "./headers" };
    } from "./lib/linkedlist"
    ```

## CLI reference

### `epine create [options] <path>`

Create a new project at the given path. This command fails if `path` already
exists. To initialize a project in a pre-existing directory, use `epine init`.

#### Arguments

`path` The path at which the project should be created.

#### Options

`--bin` Create a binary project (e.g. `corewar` executable). This is the default
    behaviour.

`--lib` Create a library project (e.g. `libcorewar.a`).

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

##### To initialize a new binary project

```sh
epine init
```

##### To initialize a new library project

```sh
epine init --lib
```

## Configuration file quick reference (`Epine.lua`)

The configuration file is written in [Lua].

[Lua]: https://lua.org

```lua
-- to import a module
local Epitech = require "epitech"

-- the Makefile template to use (see reference below)
Epitech.template "MUL_my_rpg_2019"

-- a list of values to declare at the top of the generated Makefile
config {
    NAME = "my_rpg";
}

-- a target (binary by default)
target "$(NAME)"
    -- add a single source file
    file "./src/main.c"

    -- add multiple source files
    file { "./src/menu.c", "./src/player.c" }

    -- or add files matching a pattern
    file { match "./src/*.c" }

    -- remove files
    file_remove "./src/test.c"

    -- add an include path
    include "./include"

    -- add many include paths
    include { "./headers", match "./lib/*/include" }

    -- remove include paths
    include_remove "./lib/secret/include"

    -- add compiler flags
    cflag { "-Wall", "-Wextra" }

    -- only add -g3 in debug mode
    cflag { debug "-g3" }

    -- remove a cflag
    cflag_remove { "-Wextra" }

    -- define symbols
    define { "ALLOW_WRITE", "ALLOW_MALLOC", "ALLOW_FREE" }

    -- remove defined symbols
    define_remove { "ALLOW_FREE" }

    -- bulid and link to a library built from a local target (defined below)
    use "libmy.a"

    -- build and link to a library pulled from a git repository
    use "game" from "https://github.com/arcarox/libgame.git"

    -- sometimes you may want to specify include paths to add. Epine will add
    -- ./include by default if the folder exists
    use {
        name = "list";
        include = { "./headers" };
    } from "./lib/linkedlist"

-- a static library target
target "libmy.a"
    -- specify that the target is a static library
    kind "static"

-- a binary for testing purposes
target "unit_tests"
    -- call a function adding some boilerplate target configuration
    -- it could for example setup some compiler flags or define some symbols
    Epitech.binary()

    -- add coverage support through compiler flag
    cflag "--coverage"

    -- if you want to manually list your source files without having to
    -- regenerate your Makefile everytime you edit that list, you can have the
    -- Makefile read that list from a file using the `cat` function
    file { cat "files.txt" }

    -- link to a system library
    link "criterion"

    -- add library search path
    linkdir { "./lib/libgote/bin" }

    -- remove a library search path
    linkdir_remove { "./lib/libgote/bin" }

-- an action is just a target that doesn't generate an output file (.PHONY)
action "tests_run"
    -- will make the "unit_tests" target
    with "unit_tests"
    -- run a shell command
    run "./unit_tests"

-- an action that runs the main binary
action "run"
    with "$(NAME)"
    run "./$(NAME) $(ARGS)"
```

## Makefile template quick reference

To specify a Makefile template to use, you can use the `loadtemplate` function:

```lua
loadtemplate ("epitech", {
    PROJECT_NAME = project_name;
    YEAR = function()
        return os.date("%Y")
    end;
    DESCRIPTION = "Makefile to build the project";
})
```

Corresponding template:

```Makefile
##
## EPITECH PROJECT, {{ YEAR }}
## {{ PROJECT_NAME }}
## File description:
## {{ DESCRIPTION }}
##

on_obj := echo "Compiling" $(1)
on_lib := echo "Building" $(1)
on_link := echo "Linking" $(1)

{{ EPINE_BODY }}
```
