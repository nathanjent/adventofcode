import org.junit.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.Assert.*
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.Arguments.arguments
import org.junit.jupiter.params.provider.CsvSource
import org.junit.jupiter.params.provider.MethodSource
import java.io.File
import java.util.stream.Stream

class Day06Tests {
  private val inputFile = "day06/input.txt"
  private val day = Day06()

  @ParameterizedTest
  @CsvSource(
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb, 7",
    "bvwbjplbgvbhsrlpgdmjqwftvncz, 5",
    "nppdvjthqldpwncqszvftbrmjlhg, 6",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg, 10",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw, 11",
  )
  fun example1Test(input: String, expected: Int) {
    assertEquals(expected, day.part1(input))
  }
  
  @Test
  fun part1Test() {
    val input = File(inputFile).readText()
    val expected = 1582

    assertEquals(expected, day.part1(input))
  }
  
  @ParameterizedTest
  @CsvSource(
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb, 19",
    "bvwbjplbgvbhsrlpgdmjqwftvncz, 23",
    "nppdvjthqldpwncqszvftbrmjlhg, 23",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg, 29",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw, 26",
  )
  fun example2Test(input: String, expected: Int) {
    assertEquals(expected, day.part2(input))
  }
  
  @Test
  fun part2Test() {
    val input = File(inputFile).readText()
    val expected = 3588

    assertEquals(expected, day.part2(input))
  }
}
