package aoc.kt.y2018;

import kotlin.test.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.runners.Parameterized
import org.junit.runners.Parameterized.Parameters
import org.junit.experimental.theories.DataPoints
import java.io.File

/**
 * Day 2 tests. Parameterized with expected results
 * for both methods since the data is the same.
 */
@RunWith(Parameterized::class)
class Test02(
        val input: String,
        val expectedProcess1: String,
        val expectedProcess2: String) {
    companion object {
        @JvmStatic
        @Parameters(name = "{index}: process1({0})={1}, process2({0})={2}")
        fun inputs(): Collection<Array<String>> {
            return listOf(
                    arrayOf(
                            """
                            |abcdef
                            |bababc
                            |abbcde
                            |abcccd
                            |aabcdd
                            |abcdee
                            |ababab
                            """.trimMargin(), "12", "abcde"),
                    arrayOf(
                            """
                            |abcde
                            |fghij
                            |klmno
                            |pqrst
                            |fguij
                            |axcye
                            |wvxyz
                            """.trimMargin(), "0", "fgij"),
                    arrayOf(File("../input/2018/day02.txt")
                        .readText(),
                        "7350", "wmlnjevbfodamyiqpucrhsukg")
            )
        }
    }

    /** Part 1 */
    @Ignore("Solved")
    @Test
    public fun processTest() {
        assertEquals(expectedProcess1, processBoxChecksum1(input))
    }

    /** Part 2 */
    @Ignore("Solved")
    @Test
    public fun process2Test() {
        assertEquals(expectedProcess2, processBoxChecksum2(input))
    }
}
