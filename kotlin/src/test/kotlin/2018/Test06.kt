package aoc.kt.y2018;

import kotlin.test.assertEquals
import org.junit.Test
import org.junit.Ignore
import org.junit.runner.RunWith
import org.junit.runners.Parameterized
import org.junit.runners.Parameterized.Parameters
import org.junit.experimental.theories.DataPoints
import java.io.File

/**
 * Day 6 tests. Parameterized with expected results
 * for both methods since the data is the same.
 */
@RunWith(Parameterized::class)
class Test06(
        val input: String,
        val maxDistanceTotal: Int,
        val expectedProcess1: String,
        val expectedProcess2: String) {
    companion object {
        @JvmStatic
        @Parameters(name = "{index}: process1({0})={1}, process2({0})={2}")
        fun inputs(): Collection<Array<Any>> {
            return listOf(
                    arrayOf("""
                        |1, 1
                        |1, 6
                        |8, 3
                        |3, 4
                        |5, 5
                        |8, 9
                        """.trimMargin(), 32, "17", "16"),
                    arrayOf(File("../input/2018/day06.txt")
                        .readText(), 10_000, "4233", "45290")
            )
        }
    }

    /** Part 1 */
    @Ignore("Solved")
    @Test
    public fun processPolymerTest() {
        assertEquals(expectedProcess1, processAreas1(input))
    }

    /** Part 2 */
    @Ignore("Solved")
    @Test
    public fun processPolymer2Test() {
        assertEquals(expectedProcess2, processAreas2(input, maxDistanceTotal))
    }
}
