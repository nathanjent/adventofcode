import org.junit.Test
import org.junit.Assert.*
import java.io.File

class Day01Tests {
  private val inputFile = "day01/input.txt"
  private val day = Day01()

  private val exampleInput = """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
""".split("\n")
  
  @Test
  fun example1Test() {
    val expected = 24000

    assertEquals(expected, day.part1(exampleInput))
  }
  
  @Test
  fun part1Test() {
    val input = File(inputFile).readLines()
    val expected = 66487

    assertEquals(expected, day.part1(input))
  }
  
  @Test
  fun example2Test() {
    val expected = 45000

    assertEquals(expected, day.part2(exampleInput))
  }
  
  @Test
  fun part2Test() {
    val input = File(inputFile).readLines()
    val expected = 197301

    assertEquals(expected, day.part2(input))
  }
}