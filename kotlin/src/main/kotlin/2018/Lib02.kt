package aoc.kt.y2018;

/**
 * Day 2.
 */

/** Part 1 */
fun processBoxChecksum1(input: String): String {
    val (count2s, count3s) = input.lines()
        .filter { !it.isEmpty() }
        .map { it.toCharArray() }
        .map {
            val charCounts = mutableMapOf<Char, Int>()
            it.forEach { c ->
                val count = charCounts.getOrDefault(c, 0)
                charCounts.put(c, count + 1)
            }
            Pair(if (charCounts.containsValue(2)) 1 else 0,
                if (charCounts.containsValue(3)) 1 else 0)
        }
        .fold(Pair(0, 0), {
            (count2s, count3s), (two, three) ->
            Pair(count2s + two, count3s + three)
        })

    return (count2s * count3s).toString()
}

/** Part 2 */
fun processBoxChecksum2(input: String): String {
    val output = input.lines()
        .filter { !it.isEmpty() }
        .map { it.toCharArray() }
        .toList()
    //return output.toString()
    return "42"
}
