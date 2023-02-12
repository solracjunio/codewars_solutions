kata = {}

function kata.stringToArray(s)
    local array = {}

    if s == '' then
        table.insert(array, '')
    else
        for word in string.gmatch(s, "%w+") do
            table.insert(array, word)
        end
    end

    return array

end

return kata