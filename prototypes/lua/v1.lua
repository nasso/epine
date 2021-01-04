-- binary: my_rpg

local Epitech = require "Epitech"

local pkg = {
    nasso = registry "https://github.com/nasso",
    arcarox = registry "https://github.com/arcarox",
}

config {
    NAME = "my_rpg";
}

target "$(NAME)"
    Epitech.graphics()
    deps {
        pkg.nasso "libmy",
        pkg.arcarox "libmy",
        pkg.arcarox "libgame",
        local_lib "./lib/liblinkedlist",
    }

target "unit_tests"
    Epitech.testbin {
        src = cat "files.mk";
    }

target "tests_run"
    with "unit_tests"
    run "./unit_tests"

target "run"
    with "$(NAME)"
    run "./$(NAME) $(ARGS)"
