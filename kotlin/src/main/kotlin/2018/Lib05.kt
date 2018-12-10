package aoc.kt.y2018;

/**
 * Day 5.
 */

/** Part 1 */
fun processPolymer1(input: String): String {
    var polymer = Pair(input, true)
    while (polymer.second) {
        polymer = react(polymer.first)
    }

    return polymer
        //.first.length
        .toString()
}

/** Part 2 */
fun processPolymer2(input: String): String {
    return "42"
}

fun react(input: String): Pair<String, Boolean> {
    var result = mutableListOf<Char>()
    var polymer = input.toMutableList()
    var reactionOccured = false

    while (polymer.next() != null) {
        polymer.dequeue()?.let { a ->
            if (polymer.next() != null) {
                polymer.dequeue()?.let { b ->
                    if (a.equals(b, true)) {
                        reactionOccured = true
                    } else {
                        result.push(a)
                        polymer.enqueue(b)
                    }
                }
            }
        }
    }

    val resultStr: String = result.map { it.toString() }.reduce { acc, n -> acc + n }
    return Pair(resultStr, reactionOccured)
}

fun <T> MutableList<T>.push(e: T) {
    this.add(e)
}

fun <T> MutableList<T>.dequeue(): T? {
    if (this.isNotEmpty()) {
        return this.removeAt(0)
    } else {
        return null
    }
}

fun <T> MutableList<T>.enqueue(e: T) {
    this.add(0, e)
}

fun <T> MutableList<T>.next(): T? {
    return this.getOrNull(0)
}
