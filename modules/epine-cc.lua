local cc = {}

local cleanlist = {}
local fcleanlist = {}

local function ns(...)
    local varname = "EPINE_CC"

    for _, v in ipairs({ ... }) do
        varname = varname .. "_" .. string.gsub(v, "[^%w]", "_")
    end

    return varname
end

function cc.registry(cfg)
    return function(name)
        if type(cfg.packages) == "function" then
            return cfg.packages(name)
        else
            return cfg.packages[name]
        end
    end
end

function cc.static(name)
    return function(cfg)
        local srcs = ns(name, "SRCS")
        local objs = ns(name, "OBJS")
        local cflags = ns(name, "CFLAGS")

        cleanlist[#cleanlist+1] = "rm -f " .. v(objs)
        fcleanlist[#fcleanlist+1] = "rm -f " .. name

        return {
            svar(srcs, table.concat(cfg.srcs, " "));
            svar(objs, "$(" .. srcs .. ":.c=.o)");
            svar(cflags, table.concat(cfg.cflags, " "));

            erule {
                targets = { name, v(objs) };
                prerequisites = cfg.prerequisites;
            };

            sprule {
                targets = { v(objs) };
                target_pattern = "%.o";
                prereq_patterns = { "%.c" };
                recipe = {
                    "$(CC) $(CFLAGS) " .. v(cflags) .. " -c -o $@ $<";
                };
            };

            erule {
                targets = { name };
                prerequisites = { v(objs) };
                recipe = { "$(AR) rc $@ " .. v(objs) };
            };
        }
    end
end

function cc.binary(name)
    return function(cfg)
        local srcs = ns(name, "SRCS")
        local objs = ns(name, "OBJS")
        local cflags = ns(name, "CFLAGS")
        local ldlibs = ns(name, "LDLIBS")
        local ldflags = ns(name, "LDFLAGS")

        cleanlist[#cleanlist+1] = "rm -f " .. v(objs)
        fcleanlist[#fcleanlist+1] = "rm -f " .. name

        return {
            svar(srcs, table.concat(cfg.srcs, " "));
            svar(objs, "$(" .. srcs .. ":.c=.o)");
            svar(cflags, table.concat(cfg.cflags, " "));
            svar(ldlibs, table.concat(cfg.ldlibs, " "));
            svar(ldflags, table.concat(cfg.ldflags, " "));

            erule {
                targets = { name, v(objs) };
                prerequisites = cfg.prerequisites;
            };

            sprule {
                targets = { v(objs) };
                target_pattern = "%.o";
                prereq_patterns = { "%.c" };
                recipe = {
                    "$(CC) $(CFLAGS) " .. v(cflags) .. " -c -o $@ $<";
                };
            };

            erule {
                targets = { name };
                prerequisites = { v(objs) };
                recipe = { "$(CC) -o $@ " .. v(objs, ldlibs, ldflags) };
            };
        }
    end
end

function cc.clean()
    return cleanlist
end

function cc.fclean()
    return fcleanlist
end

cc.lib = cc.registry {
    packages = function(name)
        return {
            link = { "-l" .. name };
        }
    end
}

return cc
