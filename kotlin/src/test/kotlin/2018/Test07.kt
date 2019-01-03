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
 * Day 7 tests. Parameterized with expected results
 * for both methods since the data is the same.
 */
@RunWith(Parameterized::class)
class Test07(
        val input: String,
        val expectedProcess1: String,
        val expectedProcess2: String) {
    companion object {
        @JvmStatic
        @Parameters(name = "{index}: processSteps1({0})={1}, processSteps2({0})={2}")
        fun inputs(): Collection<Array<String>> {
            return listOf(
                    arrayOf("""
                        |Step C must be finished before step A can begin.
                        |Step C must be finished before step F can begin.
                        |Step A must be finished before step B can begin.
                        |Step A must be finished before step D can begin.
                        |Step B must be finished before step E can begin.
                        |Step D must be finished before step E can begin.
                        |Step F must be finished before step E can begin.
                        """.trimMargin(), "42", "42")
                    //    ,
                    //arrayOf(File("../input/2018/day07.txt")
                    //    .readText(), "42", "42")
            )
        }
    }

    /** Part 1 */
    @Test
    @Ignore("On hold")
    public fun processStepsTest() {
        assertEquals(expectedProcess1, processSteps1(input))
    }

    /** Part 2 */
    @Test
    public fun processSteps2Test() {
        assertEquals(expectedProcess2, processSteps2(input))
    }
}
