-- this is a test file

local cc = require "epine-cc"

return {
    var("SHELL", "/bin/sh");
    var("NAME", "libcorewar.a");
    var("CFLAGS", "-Wall -Wextra -pedantic");

    action "all" {
        with = { "$(NAME)" }
    };

    action "re" {
        with = { "fclean", "all" };
    };

    cc.static "$(NAME)" {
        prerequisites = { "lib/libmy/include" };

        srcs = { find "./src/*.c" };
        cflags = { "-Iinclude", "-Ilib/libmy/include" };
    };

    cc.binary "tests.out" {
        prerequisites = { "$(NAME)", "lib/libmy/include", "lib/libmy/libmy.a" };

        srcs = { find "./tests/*.c" };
        cflags = { "-Iinclude", "-Ilib/libmy/include" };
        ldflags = { "-L.", "-Llib/libmy" };
        ldlibs = { "-lcorewar", "-lmy", "-lcriterion" };
    };

    target "lib/libmy" "lib/libmy/include" {
        "mkdir -p lib/libmy";
        "git clone https://github.com/nasso/libmy lib/libmy";
        "rm -rf lib/libmy/.git";
    };

    target "lib/libmy/libmy.a" {
        with = { "lib/libmy" };

        "$(MAKE) -C lib/libmy";
    };

    action "tests_run" {
        with = { "tests.out" };

        "./tests.out $(ARGS)";
    };

    action "clean" {
        cc.clean();
        "$(MAKE) -C lib/libmy fclean";
    };

    action "fclean" {
        cc.clean();
        cc.fclean();
        "$(MAKE) -C lib/libmy fclean";
    };
}
