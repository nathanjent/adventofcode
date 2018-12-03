package aoc.kt.y2018;

/**
 * Day 2.
 */

 data class Claim(
         val id: Int,
         val x: Int,
         val y: Int,
         val w: Int,
         val h: Int)

/** Part 1 */
fun process1(input: String): String {
    val output = input.lines()
        .filter { !it.isEmpty() }
        .map { it.split(" @ ", ": ") }
        .map {
            val idStr = it.get(0)
            val id = idStr.substring(1, idStr.length-1)
                .toInt()
            val position = it.get(1).split(',')
            val size = it.get(2).split('x')
            val x = position.get(0).toInt()
            val y = position.get(1).toInt()
            val w = size.get(0).toInt()
            val h = size.get(1).toInt()
            Claim(id, x, y, w, h)
        }
        .toList()

    return output.toString()
}

/** Part 2 */
fun process2(input: String): String {
    return "42"
}
