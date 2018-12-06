package aoc.kt.y2018;

/**
 * Day 4.
 */

data class Message(val day: Int, val hour: Int, val minute: Int, val text: String)

/** Part 1 */
fun processRepose1(input: String): String {
    var output = input.lines()
        .filter { !it.isEmpty() }
        .map {
            //val id = """(-([0-9][0-9]))|(:[0-9]\])|(\#[0-9]+)|(asleep)|(wake)""".toRegex()
            val (_, _, day, hour, minute) = "[^#][0-9]+".toRegex()
                .findAll(it)
                .map { it.value }
                .map { "[0-9]+".toRegex().find(it) }
                .filter { it != null }
                .map { it.value }
                .map { it.toInt() }
                .toList()

            val text = ""

            Message(day, hour, minute, text)
        }
        .flatMap { it.toList() }
        .toList()
    return output.toString()
}

/** Part 2 */
fun processRepose2(input: String): String {
    return "42"
}
