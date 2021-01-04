local Epitech = require "Epitech"

config {
    NAME = "my_defender"
}

target "$(NAME)"
    Epitech.cbinary { "write", "malloc", "free" }
    link "criterion"
    link "my"
    link "gote"
    link "dragon"
    link "csfml-system"

target "unit_tests"
    type "binary"
    src { wildcard "./src/*.c" }
    include { "./include" }

target "tests_run"
    deps "unit_tests"
