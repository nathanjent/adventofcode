local aoc = {}

local abs = math.abs

function check(levels)
      local prevLevel = nil
      local safe = true
      local ascending = nil

      for _,currLevel in ipairs(levels) do
         if prevLevel ~= nil then
            local diff = currLevel - prevLevel
            local asc = diff > 0
            if ascending == nil then
               ascending = asc
            end

            diff = abs(diff)
            if diff == 0 or asc ~= ascending or diff > 3 then
               --print(prevLevel .. " " .. currLevel)
               safe = false
               break
            end
            ascending = asc
         end
         prevLevel = currLevel
      end

      return safe
end

function aoc:part1(input)
   local result = 0

   for line in string.gmatch(input, "([^\n]+)\n?") do
      --print(line)
      local levels = {}
      for level in string.gmatch(line, "((%d+)%s?)") do
         table.insert(levels, level)
      end

      local safe = check(levels)

      if safe then
         result = result + 1
      end
   end

   return result
end

function aoc:part2(input)
   local result = 0

   for line in string.gmatch(input, "([^\n]+)\n?") do
      --print(line)
      local levels = {}
      for level in string.gmatch(line, "((%d+)%s?)") do
         table.insert(levels, level)
      end

      local safe = check(levels)

      -- check if safe if one level removed
      for i, level in ipairs(levels) do
         -- remove one level
         local r = table.remove(levels, i)
         safe = check(levels)
         if safe then
            break
         end

         -- put it back
         table.insert(levels, i, r)
         i = i + 1
      end

      if safe then
         result = result + 1
      end
   end

   return result
end
return aoc
