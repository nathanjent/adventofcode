package aoc.kt.y2018;

import kotlin.test.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.runners.Parameterized
import org.junit.runners.Parameterized.Parameters
import org.junit.experimental.theories.DataPoints
import java.io.File

/**
 * Day 1 tests. Parameterized with expected results
 * for both methods since the data is the same.
 */
@RunWith(Parameterized::class)
class Test01(
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
                            |+1
                            |-2
                            |+3
                            |+1
                            """.trimMargin(), "3", "2"),
                    arrayOf(
                            """
                            |+1
                            |+1
                            |+1
                            """.trimMargin(),"3", "It's over 9000!"),
                    arrayOf(
                            """
                            |+1
                            |+1
                            |-2
                            """.trimMargin(), "0", "0"),
                    arrayOf(
                            """
                            |-1
                            |-2
                            |-3
                            """.trimMargin() ,"-6", "It's over 9000!"),
                    arrayOf(
                            """
                            |+1
                            |-1
                            """.trimMargin() ,"0", "0"),
                    arrayOf(
                            """
                            |+3
                            |+3
                            |+4
                            |-2
                            |-4
                            """.trimMargin() ,"4", "10"),
                    arrayOf(
                            """
                            |-6
                            |+3
                            |+8
                            |+5
                            |-6
                            """.trimMargin(), "4", "5"),
                    arrayOf(
                            """
                            |+7
                            |+7
                            |-2
                            |-7
                            |-4
                            """.trimMargin(), "1", "14"),
                    arrayOf(
                            File("../input/2018/day01.txt")
                            .readText(),
                            "466", "750")
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
