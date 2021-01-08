-- Epitech.lua
local Epitech = {}

function Epitech.template(project_name)
    loadtemplate ("epitech", {
        PROJECT_NAME = project_name;
        YEAR = function()
            return os.date("%Y")
        end;
        DESCRIPTION = "Makefile to build the project";
    })
end

function Epitech.binary(srcs, exclude_srcs)
    include "./include"

    if srcs then
        file (srcs)
    else
        file { match "./src/*.c" }
    end

    if exclude_srcs then
        file_remove (exclude_srcs)
    end

    cflag "-Wall"
    cflag "-Wextra"
    cflag (debug "-g3")
end

function Epitech.graphics(...)
    Epitech.binary(...)

    link "csfml-system"
    link "csfml-graphics"
    link "csfml-audio"
end

return Epitech
-------------------------------------------------------------

-- binary: my_rpg

local Epitech = require "epitech"

Epitech.template "MUL_my_rpg_2019"

config {
    NAME = "my_rpg";
}

target "$(NAME)"
    Epitech.graphics()
    use "libmy.a" -- from "./lib/libmy" make "libmy"
    use "game" from "https://github.com/arcarox/libgame.git"
    use {
        name = "list";
        include = { "./headers" };
    } from "./lib/linkedlist"

target "unit_tests"
    Epitech.binary({ cat "files.txt" }, { "./src/main.c" })
    link "criterion"
    cflag "--coverage"

action "tests_run"
    with "unit_tests"
    run "./unit_tests"

action "run"
    with "$(NAME)"
    run "./$(NAME) $(ARGS)"
