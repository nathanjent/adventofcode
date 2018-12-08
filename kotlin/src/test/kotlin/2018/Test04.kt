package aoc.kt.y2018;

import kotlin.test.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.runners.Parameterized
import org.junit.runners.Parameterized.Parameters
import org.junit.experimental.theories.DataPoints
import java.io.File

/**
 * Day 4 tests. Parameterized with expected results
 * for both methods since the data is the same.
 */
@RunWith(Parameterized::class)
class Test04(
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
                            |[1518-11-01 00:00] Guard #10 begins shift
                            |[1518-11-01 00:05] falls asleep
                            |[1518-11-01 00:25] wakes up
                            |[1518-11-01 00:30] falls asleep
                            |[1518-11-01 00:55] wakes up
                            |[1518-11-01 23:58] Guard #99 begins shift
                            |[1518-11-02 00:40] falls asleep
                            |[1518-11-02 00:50] wakes up
                            |[1518-11-03 00:05] Guard #10 begins shift
                            |[1518-11-03 00:24] falls asleep
                            |[1518-11-03 00:29] wakes up
                            |[1518-11-04 00:02] Guard #99 begins shift
                            |[1518-11-04 00:36] falls asleep
                            |[1518-11-04 00:46] wakes up
                            |[1518-11-05 00:03] Guard #99 begins shift
                            |[1518-11-05 00:45] falls asleep
                            |[1518-11-05 00:55] wakes up
                            """.trimMargin(), "240", "42"),
                    arrayOf(File("../input/2018/day04.txt")
                        .readText(),
                        "42", "42")
            )
        }
    }

    /** Part 1 */
    @Test
    public fun processTest() {
        assertEquals(expectedProcess1, processRepose1(input))
    }

    /** Part 2 */
    @Test
    public fun process2Test() {
        assertEquals(expectedProcess2, processRepose2(input))
    }
}
