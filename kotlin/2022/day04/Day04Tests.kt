import org.junit.Test
import org.junit.Assert.*
import java.io.File

class Day04Tests {
  private val inputFile = "day04/input.txt"
  private val day = Day04()

  private val exampleInput = """
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
""".split("\n")
  
  @Test
  fun example1Test() {
    val expected = 2

    assertEquals(expected, day.part1(exampleInput))
  }
  
  @Test
  fun part1Test() {
    val input = File(inputFile).readLines()
    val expected = 605

    assertEquals(expected, day.part1(input))
  }
  
  @Test
  fun example2Test() {
    val expected = 4

    assertEquals(expected, day.part2(exampleInput))
  }
  
  @Test
  fun part2Test() {
    val input = File(inputFile).readLines()
    val expected = 914

    assertEquals(expected, day.part2(input))
  }
}