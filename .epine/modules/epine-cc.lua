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

        cleanlist[#cleanlist+1] = "rm -f " .. var(objs)
        fcleanlist[#fcleanlist+1] = "rm -f " .. name

        return {
            epine.svar(srcs, table.concat(cfg.srcs, " "));
            epine.svar(objs, "$(" .. srcs .. ":.c=.o)");
            epine.svar(cflags, table.concat(cfg.cflags, " "));

            epine.erule {
                targets = { name, var(objs) };
                prerequisites = cfg.prerequisites;
            };

            epine.sprule {
                targets = { var(objs) };
                target_pattern = "%.o";
                prereq_patterns = { "%.c" };
                recipe = {
                    "$(CC) $(CFLAGS) " .. var(cflags) .. " -c -o $@ $<";
                };
            };

            epine.erule {
                targets = { name };
                prerequisites = { var(objs) };
                recipe = { "$(AR) rc $@ " .. var(objs) };
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

        cleanlist[#cleanlist+1] = "rm -f " .. var(objs)
        fcleanlist[#fcleanlist+1] = "rm -f " .. name

        return {
            epine.svar(srcs, table.concat(cfg.srcs, " "));
            epine.svar(objs, "$(" .. srcs .. ":.c=.o)");
            epine.svar(cflags, table.concat(cfg.cflags, " "));
            epine.svar(ldlibs, table.concat(cfg.ldlibs, " "));
            epine.svar(ldflags, table.concat(cfg.ldflags, " "));

            epine.erule {
                targets = { name, var(objs) };
                prerequisites = cfg.prerequisites;
            };

            epine.sprule {
                targets = { var(objs) };
                target_pattern = "%.o";
                prereq_patterns = { "%.c" };
                recipe = {
                    "$(CC) $(CFLAGS) " .. var(cflags) .. " -c -o $@ $<";
                };
            };

            epine.erule {
                targets = { name };
                prerequisites = { var(objs) };
                recipe = { "$(CC) -o $@ " .. vars(objs, ldlibs, ldflags) };
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
