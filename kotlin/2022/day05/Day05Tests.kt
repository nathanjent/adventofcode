import org.junit.Test
import org.junit.Assert.*
import java.io.File

class Day05Tests {
  private val inputFile = "day05/input.txt"
  private val day = Day05()

  private val exampleInput = """
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
""".split("\n")
  
  @Test
  fun example1Test() {
    val expected = "CMZ"

    assertEquals(expected, day.part1(exampleInput))
  }
  
  @Test
  fun part1Test() {
    val input = File(inputFile).readLines()
    val expected = "PTWLTDSJV"

    assertEquals(expected, day.part1(input))
  }
  
  @Test
  fun example2Test() {
    val expected = "MCD"

    assertEquals(expected, day.part2(exampleInput))
  }
  
  @Test
  fun part2Test() {
    val input = File(inputFile).readLines()
    val expected = "WZMFVGGZP"

    assertEquals(expected, day.part2(input))
  }
}