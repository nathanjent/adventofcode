package aoc.kt.y2018;

/**
 * Day 7.
 */

/** Part 1 */
fun processSteps1(input: String): String {
    val stepMatch = " [A-Z] ".toRegex()
    val stepMap = input.lines()
        .map {
            val (first, second) = stepMatch.findAll(it)
                .map { it.value }
                .toList()
            Pair(first, second)
        }
        .groupBy { it.first }
        .map { it.key to it.value.map { p -> p.second } }

    return stepMap
        .toString()
}

/** Part 2 */
fun processSteps2(input: String): String {
    return "42"
}
