local function tokentag(tag)
    return function(c)
        return {
            t = tag;
            c = c;
        }
    end
end

br = tokentag "Break" ()
comment = tokentag "Comment"
directive = tokentag "Directive"

function vardef(flavor)
    return function(name, value)
        return
            tokentag "Vardef" {
                t = flavor;
                c = {
                    name = name;
                    value = value;
                };
            }
    end
end

var = vardef "Recursive"
svar = vardef "Simple"
cvar = vardef "Conditional"
shvar = vardef "Shell"

erule = tokentag "ExplicitRule"
prule = tokentag "PatternRule"
sprule = tokentag "StaticPatternRule"

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
                recipe = cfg.run;
            };

            phony(name);
        }
    end
end

function make(cmd)
    return "$(MAKE)" .. cmd
end
