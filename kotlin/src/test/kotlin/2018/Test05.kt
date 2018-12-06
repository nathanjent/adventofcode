package aoc.kt.y2018;

import kotlin.test.assertEquals
import org.junit.Test
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
                    arrayOf(
                            """
                            |dabAcCaCBAcCcaDA
                            """.trimMargin(), "10", "42")
                    //        ,
                    //arrayOf(File("../input/2018/day05.txt")
                    //    .readText(), "42", "42")
            )
        }
    }

    /** Part 1 */
    @Test
    public fun processPolymerTest() {
        assertEquals(expectedProcess1, processPolymer1(input))
    }

    /** Part 2 */
    @Test
    public fun processPolymer2Test() {
        assertEquals(expectedProcess2, processPolymer2(input))
    }
}
