require 'busted.runner'()

local solution = require'solution'
local x = kata.stringToArray

function test(input, expected)
    it('Testing for "'..input..'"', function () assert.are.same(expected, x(input))  end)
end

describe('Sample Test', function ()
    test('Robin Sing', {'Robin', 'Sing'})
    test('CodeWars', {'CodeWars'})
    test('I love tables they are my favorite', {'I', 'love', 'tables', 'they', 'are', 'my', 'favorite'})
    test('1 2 3', {'1', '2', '3'})
    test('', {''})
end)