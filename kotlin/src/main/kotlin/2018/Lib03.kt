package aoc.kt.y2018;

/**
 * Day 2.
 */

data class Point(
        val x: Int,
        val y: Int) {
            var claims: MutableSet<Claim> = mutableSetOf()
        }

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
                    points.add(Point(x1, y1))
                }
            }
            Claim(id, points)
        }
        .toList()

    val overlapMap = mutableMapOf<Point,Int>()
    for (claim in claims) {
        for (other in claims) {
            if (claim.id == other.id) {
                continue
            }
            for (claimPoint in claim.points) {
                for (otherPoint in other.points) {
                    if (claimPoint.x == otherPoint.x &&
                        claimPoint.y == otherPoint.y) {
                            claimPoint.claims.add(claim)
                            claimPoint.claims.add(other)
                        val count = overlapMap.getOrDefault(claimPoint, 0)
                        overlapMap.put(claimPoint, count + 1)
                    }
                }
            }
        }
    }

    var overlapCount = overlapMap.filter { it.value > 1 } .count()

    //return overlapMap.map { (point, value) ->
    //    val claims = point.claims.map { it.id }.toString()
    //    val x = point.x
    //    val y = point.y
    //    "{ ($x, $y), claims=$claims, value=$value }\n"}.toString() +
    //        " : " + claims.map {
    //            val id = it.id
    //            val points = it.points.map {
    //                val x = it.x
    //                val y = it.y
    //                "\n\t($x, $y)"}.toString()
    //            "{ id=$id, points=$points }\n" }
    return overlapCount.toString()
}

/** Part 2 */
fun process2(input: String): String {
    return "42"
}
