import org.junit.Test
import org.junit.Assert.*
import java.io.File

class Day03Tests {
  private val inputFile = "day03/input.txt"
  private val day = Day03()

  private val exampleInput = """
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
""".split("\n")
  
  @Test
  fun example1Test() {
    val expected = 157

    assertEquals(expected, day.part1(exampleInput))
  }
  
  @Test
  fun part1Test() {
    val input = File(inputFile).readLines()
    val expected = 7568

    assertEquals(expected, day.part1(input))
  }
  
  @Test
  fun example2Test() {
    val expected = 70

    assertEquals(expected, day.part2(exampleInput))
  }
  
  @Test
  fun part2Test() {
    val input = File(inputFile).readLines()
    val expected = 2780

    assertEquals(expected, day.part2(input))
  }
}