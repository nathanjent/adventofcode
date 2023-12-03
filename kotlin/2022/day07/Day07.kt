class Day07 {

  data class File(val name: String, val size: Int)

  class FileSystem {
    private var pwd: String = "/"
    private val fileSystemStructure = mutableMapOf(
      Pair(pwd, mutableMapOf<String, Int>()),
    )

    fun cd(path: String) {
      if (path == "..") {
        pwd = pwd.substring(0, pwd.lastIndexOf("/"))
      } else if (path != pwd) {
        pwd += "${if (pwd != "/") "/" else ""}${path}"
      }
    }

    fun ls(): List<File> {
      return fileSystemStructure.getOrDefault(pwd, mutableMapOf()).map { File(it.key, it.value) }
    }

    fun add(path: String, size: Int = 0) {
      val dirContents = fileSystemStructure.getOrPut(pwd) { mutableMapOf() }
      dirContents[path] = size
    }

    fun du(fullPath: String): Int {
      val dirContents = fileSystemStructure[fullPath]
      return dirContents?.map {
        if (it.value == 0) {
          du(it.key)
        } else {
          it.value
        }
      }?.sum() ?: 0
    }
  }

  fun part1(input: Iterable<String>): Int {
    val lines = input.toList()
    val fileSystem = FileSystem()
    var cursor = 0

    main@ while (cursor < lines.size) {
      var line = lines[cursor++]
      if (line.isBlank()) continue
      val args = line.split(" ")

      if (args[0] == "$") {
        if (args[1] == "cd") {
          fileSystem.cd(args[2])
        }

        if (args[1] == "ls") {
          ls@ while (cursor < lines.size) {
            line = lines[cursor++]
            if (line.isBlank()) continue@ls
            val words = line.split(" ")

            if (words[0] == "dir") {
              fileSystem.add(words[1])
            } else if (words[0] == "$") {
              cursor--
              continue@main
            } else {
              val size = words[0].toInt()
              fileSystem.add(words[1], size)
            }
          }
        }
      }
    }

    val f = fileSystem.ls()

    return 0
  }

  fun part2(input: Iterable<String>): Int {
    return 0
  }
}
