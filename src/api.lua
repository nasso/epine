epine = {}

local function tokentag(tag)
    return function(c)
        return {
            t = tag;
            c = c;
        }
    end
end

local function vardef(flavor)
    return function(name, value)
        return tokentag "Vardef" {
            t = flavor;
            c = {
                name = name;
                value = value;
            };
        }
    end
end

epine.br = tokentag "Break" ()
epine.comment = tokentag "Comment"
epine.directive = tokentag "Directive"
epine.var = vardef "Recursive"
epine.svar = vardef "Simple"
epine.cvar = vardef "Conditional"
epine.shvar = vardef "Shell"
epine.erule = tokentag "ExplicitRule"
epine.prule = tokentag "PatternRule"
epine.sprule = tokentag "StaticPatternRule"

-- public utils (global!)

function var(name, ...)
    if ... then
        return epine.var(name, ...)
    else
        return "$(" .. name .. ")"
    end
end

function vars(...)
    return "$(" .. table.concat({...}, ") $(") .. ")"
end

function phony(...)
    return epine.erule {
        targets = { ".PHONY" };
        prerequisites = { ... };
    };
end

function find(str)
    return "$(shell find -path '" .. str .. "')"
end

function make(cmd)
    return "$(MAKE)" .. cmd
end

function action(name)
    return function(cfg)
        return {
            epine.erule {
                targets = { name };
                prerequisites = cfg.prerequisites;
                recipe = cfg.run;
            };

            phony(name);
        }
    end
end

function target(...)
    local targets = {}

    local function nxt(name_or_cfg, ...)
        if type(name_or_cfg) == "string" then
            local name = name_or_cfg

            for _, v in ipairs({ name, ... }) do
                assert(type(v) == "string", "inconsistent arguments")
                targets[#targets+1] = v
            end

            return nxt
        elseif type(name_or_cfg) == "table" then
            local cfg = name_or_cfg
            local recipe = {}

            for _, v in ipairs(cfg) do
                recipe[#recipe+1] = v
            end

            return epine.erule {
                targets = targets;
                prerequisites = cfg.needs;
                recipe = recipe;
            }
        else
            error ("invalid argument: " .. name_or_cfg)
        end
    end

    return nxt(...)
end
