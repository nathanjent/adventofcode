package aoc.kt.y2018;

/**
 * Day 2.
 */

data class Point(
        val x: Int,
        val y: Int)

/** Part 1 */
fun process1(input: String): String {
    val claimMap = mutableMapOf<Point, MutableList<Int>>()
    input.lines()
        .filter { !it.isEmpty() }
        .map { it.split(" @ ", ": ") }
        .forEach {
            val idStr = it.get(0)
            val position = it.get(1).split(',')
            val size = it.get(2).split('x')
            val id = idStr.substring(1 until idStr.length).toInt()
            val x = position.get(0).toInt()
            val y = position.get(1).toInt()
            val w = size.get(0).toInt()
            val h = size.get(1).toInt()
            for (x1 in x until x+w) {
                for (y1 in y until y+h) {
                    val key = Point(x1, y1)
                    val claimList = claimMap.getOrDefault(key, mutableListOf<Int>())
                    claimList.add(id)
                    claimMap.put(key, claimList)
                }
            }
        }

    var overlapCount = claimMap.filter { it.value.count() > 1 } .count()

    return overlapCount.toString()
}

/** Part 2 */
fun process2(input: String): String {
    return "42"
}
