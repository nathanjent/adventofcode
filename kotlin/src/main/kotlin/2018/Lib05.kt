package aoc.kt.y2018;

/**
 * Day 5.
 */

/** Part 1 */
fun processPolymer1(input: String): String {
    val output = input.toCharArray()
    .forEachIndexed { i, c ->
        if (i != 0) {
            var reacting = true
            var range = 0..0
            while (reacting) {
                var offset = 0
                if (reactionOccurs(c, input.get(-1))) {
                } else {
                    reacting = false
                }
            }
        }
    }
    return output.toString()
}

/** Part 2 */
fun processPolymer2(input: String): String {
    return "42"
}

fun reactionOccurs(char: Char, prev: Char): Boolean {
    return false
}
