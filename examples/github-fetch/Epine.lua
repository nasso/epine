local cc = require "@nasso/epine-cc/v0.1.0-alpha"

cc.cflags = {"-Wall", "-Wextra", "-pedantic"}

return {
    epine.var("CFLAGS", "-g3"),
    epine.br,
    action "all" {
        prerequisites = {"libmy.a"}
    },
    epine.br,
    cc.static "libmy.a" {
        srcs = {"./src/my_putstr.c", "./src/my_printf.c"},
        incdirs = {"include"},
        defines = {
            {
                "MY_ALLOW_MALLOC",
                "MY_ALLOW_FREE"
            },
            ["MY_FAKE_MALLOC_FAILURE"] = 16
        }
    },
    epine.br,
    cc.binary "unit_tests" {
        prerequisites = {"libmy.a"},
        srcs = {"tests/test.c"},
        incdirs = {"include"},
        libs = {"my", "criterion"},
        libdirs = {"."}
    },
    epine.br,
    action "tests_run" {
        prerequisites = {"unit_tests"},
        "./unit_tests"
    },
    epine.br,
    action "clean" {
        rm(cc.cleanlist)
    },
    epine.br,
    action "fclean" {
        rm(cc.cleanlist),
        rm("libmy.a", "unit_tests")
    },
    epine.br,
    action "re" {
        prerequisites = {"fclean", "all"}
    }
}
