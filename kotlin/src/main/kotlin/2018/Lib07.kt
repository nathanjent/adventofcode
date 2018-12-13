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
        .map { it.key to Pair(it.value.map { p -> p.second }, false) }
        .toMap()

    val output = mutableListOf<String>()

    // Find order
    with(output) {
        var current = stepMap.findNext()
        while (current != null) {
            current = stepMap.findNext(current)
            if (current != null) {
                add(current)
            }
        }
    }

    return output
        .toString()
}

/** Part 2 */
fun processSteps2(input: String): String {
    return "42"
}

fun Map<String, Pair<List<String>, Boolean>>.hasNext(current: String? = null): Boolean {
    return current != null && this.containsKey(current)
}

fun Map<String, Pair<List<String>, Boolean>>.findNext(current: String? = null): String? {
    if (current == null) return this.findRoot()

    return this.get(current)?.first?.minBy { it }
}

fun Map<String, Pair<List<String>, Boolean>>.findRoot(): String {
    return this.keys.filterNot { step ->
        this.values.any { nextSteps -> nextSteps.first.contains(step) }
    }.first()
}
