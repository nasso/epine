local plugins = {}

local function get_lang_plugin(lang)
    return plugins[lang]
end

plugins.action = {}

function plugins.action.init_props()
    return {
        steps = {};
    }
end

function plugins.action.add_target(name, props)
    epine.add_target({
        name = name;
        phony = true;
        prerequisites = {};
        rules = props.steps;
    })
end

plugins.c = {}

function plugins.c.init_props()
    return {
        kind = "binary";
        src = {};
        include = {};
    }
end

function plugins.c.add_target(name, props)
    local t = {
        name = name;
        phony = false;
        prerequisites = {};
        rules = { "gcc $(CFLAGS) $(SRC) -o $@" };
    }

    table.insert(t.prerequisites, "SRC :=")
    table.insert(t.prerequisites, "CFLAGS :=")

    for _, f in ipairs(props.src) do
        table.insert(t.prerequisites, "SRC += " .. f)
    end

    for _, f in ipairs(props.include) do
        table.insert(t.prerequisites, "CFLAGS += -I '" .. f .. "'")
    end

    table.insert(t.prerequisites, "OBJ := $(SRC:.c=.o)")

    epine.add_target(t)
end

local ct = nil

local function flush_target()
    if ct then
        local p = get_lang_plugin(ct.lang)

        p.add_target(ct.name, ct.props)
    end
end

function epine.on_end()
    flush_target()
end

function target(name, lang, props)
    flush_target()
    ct = {
        name = name;
        lang = lang or "c";
        props = props;
    }
end

function lang(name)
    ct.lang = name
end

local function ensure_props_created()
    if not ct.props then
        local p = get_lang_plugin(ct.lang)

        ct.props = p.init_props()
    end
end

local function create_list_prop(name)
    local function apply(value, fn)
        if type(value) == "table" then
            for i, v in ipairs(value) do
                apply(v, fn)
            end

            return
        end

        assert(type(value) == "string")

        ensure_props_created()
        assert(
            ct.props[name],
            "no " .. name .. " list in \"" .. ct.lang .. "\" targets"
        )

        fn(value)
    end

    local function add(value)
        apply(value, function(value)
            table.insert(ct.props[name], value)
        end)
    end

    local function remove(value)
        apply(value, function(value)
            local index = nil

            repeat
                index = nil

                for i, v in ipairs(ct.props[name]) do
                    if v == value then
                        index = i
                        break
                    end
                end

                if index then
                    table.remove(ct.props[name], index)
                end
            until not index
        end)
    end

    return add, remove
end

function action(name)
    target(
        name,
        "action",
        {
            steps = {};
        }
    )
end

file, file_remove = create_list_prop("src")
include, include_remove = create_list_prop("include")
run = create_list_prop("steps")

function kind(name)
    ensure_props_created()
    ct.props.kind = name
end

function match(pattern)
    return "$(wildcard " .. pattern .. ")"
end

local function project(name, k)
    target(name, "c", nil)
    kind(k)
end

function binary(name) project(name, "binary") end
function static(name) project(name, "static") end
function shared(name) project(name, "shared") end
