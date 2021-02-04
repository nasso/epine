local cc = require "@nasso/epine-cc/v0.2.0-alpha"

return {
    epine.var("CC", "g++"):targets("MyGKrellm"),
    epine.br,
    -- supported target types: cc.binary, cc.static
    -- planned: cc.shared
    cc.binary "MyGKrellm" {
        -- target prerequisites
        prerequisites = {"./lib/libjzon.a"},
        -- language ("C" (default) or "C++")
        lang = "C++",
        -- source files
        srcs = {find "./src/*.cpp"},
        -- preprocessor flags (include dirs)
        cppflags = {"-I./include", "-I./lib/libjzon/include"},
        -- compiler flags
        cxxflags = {"-Wall", "-Wextra"},
        -- libraries
        ldlibs = {
            "-lsfml-graphics",
            "-lsfml-window",
            "-lsfml-system",
            "-ljzon"
        },
        -- lib dirs and other linker flags
        ldflags = {"-L./lib"}
    },
    -- [...]

    action "clean" {
        -- cc.cleanlist represents all the files generated during compilation
        -- it does NOT contain the final executable or library
        rm(cc.cleanlist)
    }
}
