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
 * Day 5 tests. Parameterized with expected results
 * for both methods since the data is the same.
 */
@RunWith(Parameterized::class)
class Test05(
        val input: String,
        val expectedProcess1: String,
        val expectedProcess2: String) {
    companion object {
        @JvmStatic
        @Parameters(name = "{index}: processPolymer1({0})={1}, processPolymer2({0})={2}")
        fun inputs(): Collection<Array<String>> {
            return listOf(
                    arrayOf("aA", "0", "42"),
                    arrayOf("abBA", "0", "42"),
                    arrayOf("abAB", "4", "42"),
                    arrayOf("aabAAB", "6", "42"),
                    arrayOf("dabAcCaCBAcCcaDA", "10", "42"),
                    arrayOf("baaRRBQqbeEFF", "7", "42"),
                    arrayOf(File("../input/2018/day05.txt")
                        .readText(), "42", "42")
                        // too high 14863
                        // too low 10387
                        // not 10775
            )
        }
    }

    /** Part 1 */
    @Test
    @Ignore("On hold")
    public fun processPolymer1Test() {
        assertEquals(expectedProcess1, processPolymer1(input))
    }

    /** Part 2 */
    @Test
    public fun processPolymer2Test() {
        assertEquals(expectedProcess2, processPolymer2(input))
    }
}
