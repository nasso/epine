local function flatten(t)
    local flat = {}

    for _, v in ipairs(t) do
        if type(v) == "table" and #v ~= 0 then
            for _, vv in ipairs(flatten(v)) do
                flat[#flat+1] = vv
            end
        else
            flat[#flat+1] = v
        end
    end

    return flat
end

local function tokentag(tag)
    return function(t)
        t.tag = tag

        return t
    end
end

local function xvar(flavor)
    return function(name, value)
        return
            vardef {
                flavor = flavor;
                name = name;
                value = value;
            };
    end
end

function comment(s)
    return {
        tag = "comment";
        text = s;
    }
end

vardef = tokentag "vardef"
erule = tokentag "erule"
irule = tokentag "irule"
sprule = tokentag "sprule"
directive = tokentag "directive"
br = tokentag "br" {}

var = xvar "recursive"
svar = xvar "simple"
cvar = xvar "conditional"
shvar = xvar "shell"

-- utils
function v(...)
    return "$(" .. table.concat({...}, ") $(") .. ")"
end

function phony(...)
    return {
        erule {
            targets = { ".PHONY" };
            prerequisites = { ... };
        };
    }
end

function find(str)
    return "$(shell find -path '" .. str .. "')"
end

function action(name)
    return function(cfg)
        return {
            erule {
                targets = { name };
                prerequisites = cfg.prerequisites;
                recipe = cfg.run and flatten(cfg.run);
            };

            phony(name);
        }
    end
end

function make(cmd)
    return "$(MAKE)" .. cmd
end
