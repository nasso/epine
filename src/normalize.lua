--[[ rPrint(struct, [limit], [indent])   Recursively print arbitrary data.
	Set limit (default 100) to stanch infinite loops.
	Indents tables as [KEY] VALUE, nested tables as [KEY] [KEY]...[KEY] VALUE
	Set indent ("") to prefix each line:    Mytable [KEY] [KEY]...[KEY] VALUE
--]]
local function rPrint(s, l, i) -- recursive Print (structure, limit, indent)
    l = (l) or 100
    i = i or "" -- default item limit, indent string
    if (l < 1) then
        print "ERROR: Item limit reached."
        return l - 1
    end
    local ts = type(s)
    if (ts ~= "table") then
        print(i, ts, s)
        return l - 1
    end
    print(i, ts) -- print "table"
    for k, v in pairs(s) do -- print "[KEY] VALUE"
        l = rPrint(v, l, i .. "\t[" .. tostring(k) .. "]")
        if (l < 0) then
            break
        end
    end
    return l
end

--<
--< this script is responsible for processing the Makefile structure table
--< generated by the Epine script.
--<
--< after normalization, this table must be a flat array containing only
--< "makefile things".
--<
--< each of those "makefile thing" mustn't contain array tables with a depth
--< level greater than 1
--<

local function is_array(t)
    if type(t) ~= "table" then
        return false
    end

    for k, _ in pairs(t) do
        if type(k) ~= "number" then
            return false
        end
    end

    return true
end

local function flatten(t)
    local flat = {}

    for _, v in ipairs(t) do
        if is_array(v) then
            local vflat = flatten(v)

            if vflat then
                for _, vv in ipairs(vflat) do
                    flat[#flat + 1] = vv
                end
            end
        else
            flat[#flat + 1] = v
        end
    end

    if #flat > 0 then
        return flat
    else
        return nil
    end
end

local function normalize_thing(thing)
    --< the tag must be a string >--
    --< !currently not checking its value! >--
    assert(type(thing.t) == "string")

    --< early return for simple things like br and comments >--
    if type(thing.c) ~= "table" then
        return {t = thing.t, c = thing.c}
    end

    --< the funny >--
    local norm = {t = thing.t, c = {}}

    for k, v in pairs(thing.c) do
        if type(v) == "table" then
            if is_array(v) then
                norm.c[k] = flatten(v)
            else
                norm.c[k] = v
            end
        else
            norm.c[k] = v
        end
    end

    return norm
end

local function normalize_mkdef(def)
    local norm = {}

    --< the definition is always a table >--
    assert(type(def) == "table")

    for _, v in ipairs(def) do
        --< the definition only contains other tables >--
        assert(type(v) == "table")

        if v.t then --< its a thing!
            norm[#norm + 1] = normalize_thing(v)
        elseif is_array(v) then --< normalize "sub-definitions" recursively
            local sub = normalize_mkdef(v)

            --< flatten them into the main definition >--
            for _, sv in ipairs(sub) do
                norm[#norm + 1] = sv
            end
        else
            --< its... not a thing? >--
        end
    end

    return norm
end

return function(def)
    local normalized = normalize_mkdef(def)

    -- rPrint(normalized, nil, "norm")

    return normalized
end
