-- public utils (global!)
-- these should be relatively straightforward and only contain stuff that is
-- *very* common in Epine scripts

--- flat concat
function fconcat(list, pre, sep)
    pre = pre or ""
    sep = sep or " "

    if not list then
        return ""
    end

    local words = ""

    for i, v in ipairs(list) do
        -- add a space before all but the first word
        if i > 1 then
            words = words .. sep
        end

        if type(v) == "table" then
            words = words .. fconcat(v, pre, sep)
        else
            words = words .. pre .. tostring(v)
        end
    end

    return words
end

--- Reference one or more variable by name.
-- Examples:
-- `vref("NAME") == "$(NAME)"`
function vref(...)
    return "$(" .. fconcat({...}, "", ") $(") .. ")"
end

--- The recommended way to call `make` from within your Makefile.
-- Makes use of the `$(MAKE)` implicit variable to forward any command line
-- option that was used to call the current instance of `make`.
function make(...)
    return "$(MAKE) " .. fconcat({...})
end

--- The recommended way to call `rm` from within your Makefile.
-- Makes use of the `$(RM)` implicit variable to allow replacing the program
-- called when needed. GNU make will use `rm -f` by default.
function rm(...)
    local list = fconcat({...})

    if list ~= "" then
        return "$(RM) " .. list
    else
        return {}
    end
end

--- Prepend the command with '@' to make it not print itself out.
function quiet(...)
    return "@" .. fconcat({...})
end

--- Insert a call to the `echo` command.
function echo(...)
    return quiet("echo", ...)
end

--- Call the `find` utility to search for paths matching the given glob pattern.
function find(str)
    return "$(shell find -path '" .. str .. "')"
end

epine = {}

local function tokentag(tag)
    return function(c)
        return {
            t = tag,
            c = c
        }
    end
end

local function directive(tag)
    return function(c)
        return tokentag "Directive" {
            t = tag,
            c = c
        }
    end
end

local Vardef = {}
Vardef.__index = Vardef

function Vardef:targets(...)
    self.c.targets = {...}
    return self
end

local function vardef(flavor)
    return function(name, ...)
        return setmetatable(
            tokentag "Vardef" {
                flavor = flavor,
                name = name,
                value = fconcat({...})
            },
            Vardef
        )
    end
end

epine.br = tokentag "Break"()
epine.comment = tokentag "Comment"
epine.include = directive "Include"
epine.sinclude = directive "SInclude"
epine.var = vardef "Recursive"
epine.svar = vardef "Simple"
epine.cvar = vardef "Conditional"
epine.shvar = vardef "Shell"
epine.append = vardef "Append"
epine.erule = tokentag "ExplicitRule"
epine.prule = tokentag "PatternRule"
epine.sprule = tokentag "StaticPatternRule"

--- Create a `.PHONY` rule with the given prerequisites
-- @param ... rule names to be included in the PHONY rule
function phony(...)
    return epine.erule {
        targets = {".PHONY"},
        prerequisites = {...}
    }
end

--- Create regular rule targets
--
-- Example configuration:
--
--     target "example" {
--       prerequisites = {"other"},
--       order_only_prerequisites = {"something"},
--
--       -- recipe for the target:
--       "touch example",
--     }
--
-- Will generate:
--
--     example: other | something
--         touch example
--
-- @param name name of the target
-- @param cfg configuration for the target
function target(...)
    local targets = {}

    local function nxt(name_or_cfg, ...)
        if type(name_or_cfg) == "string" then
            local name = name_or_cfg

            for _, v in ipairs({name, ...}) do
                assert(type(v) == "string", "inconsistent arguments")
                targets[#targets + 1] = v
            end

            return nxt
        elseif type(name_or_cfg) == "table" then
            local cfg = name_or_cfg
            local recipe = {}

            for _, v in ipairs(cfg) do
                recipe[#recipe + 1] = v
            end

            return epine.erule {
                targets = targets,
                prerequisites = cfg.prerequisites,
                order_only_prerequisites = cfg.order_only_prerequisites,
                recipe = recipe
            }
        else
            error("invalid argument: " .. name_or_cfg)
        end
    end

    return nxt(...)
end

--- Same as `target` but adds a `PHONY` rule for the generated rule
--
-- Example configuration:
--
--     action "example" {
--       prerequisites = {"other"},
--       order_only_prerequisites = {"something"},
--
--       -- recipe for the target:
--       "echo hello!",
--     }
--
-- Will generate:
--
--     example: other | something
--         echo hello!
--     .PHONY: example
--
-- @param name name of the target
-- @param cfg configuration for the target
function action(...)
    local targets = {}

    local function nxt(name_or_cfg, ...)
        if type(name_or_cfg) == "string" then
            local name = name_or_cfg

            for _, v in ipairs({name, ...}) do
                assert(type(v) == "string", "inconsistent arguments")
                targets[#targets + 1] = v
            end

            return nxt
        elseif type(name_or_cfg) == "table" then
            local cfg = name_or_cfg
            local recipe = {}

            for _, v in ipairs(cfg) do
                recipe[#recipe + 1] = v
            end

            return {
                epine.erule {
                    targets = targets,
                    prerequisites = cfg.prerequisites,
                    order_only_prerequisites = cfg.order_only_prerequisites,
                    recipe = recipe
                },
                phony(targets)
            }
        else
            error("invalid argument: " .. name_or_cfg)
        end
    end

    return nxt(...)
end
