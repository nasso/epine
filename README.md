# epi

`epi` is a tiny C/C++ project and package manager. Although it was primarily
built for [Epitech] students, we aim to make it available to anyone needing a
robust package manager for their own use.

[Epitech]: https://epitech.eu

## Goals

- Make the process of creating new C/C++ projects quick and easy
- Encourage code reuse
- Make it easy to maintain your own library registry (using Git)
- Simplify the building process by generating a Makefile
- Not adding any extra dependency in your project tree.
- **Make it usable by any Epitech student without them being penalized :)**

## Non-goals

- Providing a centralized package registry
- Be used as a large scale package manager (`epi` is simple)
- Replacing make
- Replacing git

## Quickstart

```
$ epi new -i
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
└── tek.toml
```

## Example usage

To run your project:
```sh
epi run -- [args]
```

To test your project:
```sh
epi test -- [args]
```

To build your project:
```sh
epi build
```

To generate a Makefile:
```sh
epi build --target=Makefile
```

## Documentation

### `epi new [options] <path>`

Create a new package at the given path. `path` mustn't already exist, use
`epi init` to initialize a package in a pre-existing directory.

#### Options

`--bin` Create a binary package (`src/main.c`). This is the default behaviour.

`--lib` Create a library package (`src/lib.c`).

`--name` _name_ Set the executable name. Defaults to the directory name.

`-i`, `--interactive` Create the project using the interactive prompt.

#### Examples

##### To create a new binary package
```sh
epi new <path>
```

##### To create a new library package
```sh
epi new --lib <path>
```

### `epi build [options]`

Generate the Makefile, then build your program.

#### Options

`--target` _target_ Build the specified target: `all` (default), `tests`,
    `Makefile`.

### `epi test [-- args]`

Generate the Makefile, build your program and run the tests.

### `epi init [options] <path>`

#### Options

`--bin` Initialize a binary package. This is the default behaviour.

`--lib` Initialize a library package.

`--name` _name_ Set the executable name. Defaults to the directory name.

### `epi run [-- args]`

Generate the Makefile, build your program program. This is equivalent to running
`epi build && ./<executable> [args]`.

## Manifest reference (`tek.toml`)

```toml
[package]
registry = "https://github.com/nasso"

[lib]
name = "libcorewar"
src = "./corewar/src/**/*.c"
include = "./corewar/include"

[[bin]]
cc = "clang"
name = "corewar"
# src = "./src/**/*.c"
# include = "./include"
lflags = ["-lcsfml-system", "-lcsfml-graphics"]
cflags = ["-Wall", "-Wextra"]
define = ["MY_ALLOW_CALLOC"]
run-before = "epifmt check :)"
run-after = "cp $EPI_NAME ./build/"

[[bin]]
name = "asm"
src = "./asm/**/*.c"
# alternatively, src = ["./asm/*.c", "./asm/parse/instr.c"]
lflags = ["-lcsfml-system", "-lcsfml-graphics"]
cflags = ["-Wall", "-Wextra"]

[dependencies]
libmy = "0.2" # fetches https://github.com/nasso/libmy@0.2.1
libgnl = { path = "./libgnl.a" }

[[test]]
name = "unit_tests"
lflags = ["-lcriterion"]
```
