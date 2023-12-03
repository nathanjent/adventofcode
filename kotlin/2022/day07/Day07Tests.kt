import org.junit.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.Assert.*
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.Arguments.arguments
import org.junit.jupiter.params.provider.CsvSource
import org.junit.jupiter.params.provider.MethodSource
import java.io.File
import java.util.stream.Stream

class Day07Tests {
  private val inputFile = "day07/input.txt"
  private val day = Day07()

  private val exampleInput = """
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
""".split("\n")

  @Test
  fun example1Test() {
    val expected = 95437
    assertEquals(expected, day.part1(exampleInput))
  }
  
  @Test
  fun part1Test() {
    val input = File(inputFile).readLines()
    val expected = 42

    assertEquals(expected, day.part1(input))
  }

  @Test
  fun example2Test() {
    val expected = 42
    assertEquals(expected, day.part2(exampleInput))
  }
  
  @Test
  fun part2Test() {
    val input = File(inputFile).readLines()
    val expected = 42

    assertEquals(expected, day.part2(input))
  }
}
