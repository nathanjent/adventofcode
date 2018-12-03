package aoc.kt.y2018;

import kotlin.test.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.runners.Parameterized
import org.junit.runners.Parameterized.Parameters
import org.junit.experimental.theories.DataPoints
import java.io.File

/**
 * Day 3 tests. Parameterized with expected results
 * for both methods since the data is the same.
 */
@RunWith(Parameterized::class)
class Test03(
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
                            |#1 @ 1,3: 4x4
                            |#2 @ 3,1: 4x4
                            |#3 @ 5,5: 2x2
                            """.trimMargin(), "4", "42"),
                    arrayOf(File("../input/2018/day03.txt")
                        .readText(),
                        "42", "42")
            )
        }
    }

    /** Part 1 */
    @Test
    public fun processTest() {
        assertEquals(expectedProcess1, process1(input))
    }

    /** Part 2 */
    @Test
    public fun process2Test() {
        assertEquals(expectedProcess2, process2(input))
    }
}
