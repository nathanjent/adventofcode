package = "aoc-2024"
version = "dev-1"
rockspec_format = "3.0"
source = {
   url = "git+ssh://git@github.com/nathanjent/adventofcode.git",
}
description = {
   homepage = "https://github.com/nathanjent/adventofcode",
   license = "MIT",
}
dependencies = {
   "lua >= 5.1, <= 5.4",
   "busted",
}
build = {
   type = "builtin",
   modules = {
      aoc = "src/lib.lua",

      copy_directories = { "test" },
   },
}
test = {
   type = "busted",
}
