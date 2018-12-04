package aoc.kt.y2018;

/**
 * Day 2.
 */

data class Point(
        val id: Int,
        val x: Int,
        val y: Int)

data class Claim(
        val id: Int,
        val points: List<Point>)

/** Part 1 */
fun process1(input: String): String {
    val claims = input.lines()
        .filter { !it.isEmpty() }
        .map { it.split(" @ ", ": ") }
        .map {
            val idStr = it.get(0)
            val position = it.get(1).split(',')
            val size = it.get(2).split('x')
            val id = idStr.substring(1 until idStr.length).toInt()
            val x = position.get(0).toInt()
            val y = position.get(1).toInt()
            val w = size.get(0).toInt()
            val h = size.get(1).toInt()
            val points = mutableListOf<Point>()
            for (x1 in x until x+w) {
                for (y1 in y until y+h) {
                    points.add(Point(id, x1, y1))
                }
            }
            Claim(id, points)
        }
        .toList()

    val overlapMap = mutableMapOf<Pair<Int,Int>,Int>()
    for (claim in claims) {
        for (other in claims) {
            if (claim.id == other.id) {
                continue
            }
            for (claimPoint in claim.points) {
                for (otherPoint in other.points) {
                    if (claimPoint.x == otherPoint.x &&
                        claimPoint.y == claimPoint.y) {
                        val key = Pair(claimPoint.x, claimPoint.y)
                        val count = overlapMap.getOrDefault(key, 0)
                        overlapMap.put(key, count + 1)
                    }
                }
            }
        }
    }

    var overlapCount = overlapMap.filter { it.value > 1 } .count()
    return overlapMap.toString()
}

/** Part 2 */
fun process2(input: String): String {
    return "42"
}
