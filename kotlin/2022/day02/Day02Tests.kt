import org.junit.Test
import org.junit.Assert.*
import java.io.File

class Day02Tests {
  private val inputFile = "day02/input.txt"
  private val day = Day02()

  private val exampleInput = """
A Y
B X
C Z
""".split("\n")
  
  @Test
  fun example1Test() {
    val expected = 15

    assertEquals(expected, day.part1(exampleInput))
  }
  
  @Test
  fun part1Test() {
    val input = File(inputFile).readLines()
    val expected = 13682

    assertEquals(expected, day.part1(input))
  }
  
  @Test
  fun example2Test() {
    val expected = 12

    assertEquals(expected, day.part2(exampleInput))
  }
  
  @Test
  fun part2Test() {
    val input = File(inputFile).readLines()
    val expected = 12881

    assertEquals(expected, day.part2(input))
  }
}