local aoc = {}

function ParseDay1Input(input)
   local left = {}
   local right = {}
   print("\nparsing...")
   for l, r in string.gmatch(input, "(%d+)%s+(%d+)") do
      table.insert(left, l)
      table.insert(right, r)
   end
   print("parsing done")

   return left, right
end

function aoc:day1part1(input)
   local left, right = ParseDay1Input(input)

   table.sort(left)
   table.sort(right)

   local result = 0
   for i, l in ipairs(left) do
      local r = right[i]
      result = result + math.abs(l - r)
   end

   return result
end

function aoc:day1part2(input)
   local left, right = ParseDay1Input(input)

   local result = 0
   for _, l in ipairs(left) do
      local count = 0
      for _, r in ipairs(right) do
         if l == r then
            count = count + 1
         end
      end
      result = result + l * count
   end

   return result
end
return aoc
