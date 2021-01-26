-- this is a test file

local cc = require "epine-cc"

return {
    var("SHELL", "/bin/sh");
    var("NAME", "libcorewar.a");
    var("CFLAGS", "-Wall -Wextra -pedantic");

    action "all" {
        prerequisites = { "$(NAME)" }
    };

    action "re" {
        prerequisites = { "fclean", "all" };
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

    erule {
        targets = { "lib/libmy", "lib/libmy/include" };
        recipe = {
            "mkdir -p lib/libmy";
            "git clone https://github.com/nasso/libmy lib/libmy";
            "rm -rf lib/libmy/.git";
        };
    };

    erule {
        targets = { "lib/libmy/libmy.a" };
        prerequisites = { "lib/libmy" };
        recipe = {
            "$(MAKE) -C lib/libmy";
        };
    };

    action "tests_run" {
        prerequisites = { "tests.out" };

        run = { "./tests.out $(ARGS)" };
    };

    action "clean" {
        run = {
            cc.clean();
            "$(MAKE) -C lib/libmy fclean";
        };
    };

    action "fclean" {
        run = {
            cc.clean();
            cc.fclean();
            "$(MAKE) -C lib/libmy fclean";
        };
    };
}
