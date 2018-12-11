package aoc.kt.y2018;

/**
 * Day 5.
 */

/** Part 1 */
fun processPolymer1(polymer: String): String {
    var reactions = polymer.map { false }.toMutableList()
    var cursor = 0
    while (cursor < polymer.length) {
        var a = polymer.getOrNull(cursor)
        cursor++
        if (a != null) {
            var b = polymer.getOrNull(cursor)
            var backCursor = cursor - 1
            while (a?.let { x ->
                b?.let { y ->
                    x.equals(y, true) && !x.equals(y)
                }?:false
            }?:false) {
                // Note reaction
                reactions.set(backCursor, true)
                reactions.set(cursor, true)
                // Expand ignoring already reacted
                while (reactions.getOrNull(backCursor)?:false) {
                    backCursor--
                }
                cursor++
                // Setup possible next reaction
                a = polymer.getOrNull(backCursor)
                b = polymer.getOrNull(cursor)
            }
        }
    }
    return reactions.zip(polymer.asIterable())
        .filter { !it.first }
        .map { "\n" + it.second }
        .count()
        .toString()
}

/** Part 2 */
fun processPolymer2(input: String): String {
    return "42"
}
