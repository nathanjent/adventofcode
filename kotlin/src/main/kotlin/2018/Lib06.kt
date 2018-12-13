package aoc.kt.y2018;

import kotlin.math.absoluteValue

/**
 * Day 6.
 */

/** Part 1 */
fun processAreas1(input: String): String {
    val coordinateMap = input.lines()
    .mapIndexed { i, line ->
        val (xStr, yStr) = line.split(", ")
        Pair(Point(xStr.toInt(), yStr.toInt()), i)
    }
    .toMap()

    val (width, height) = coordinateMap.keys.fold(Pair(-1, -1), { acc, n ->
        Pair(if (n.x > acc.first) { n.x } else { acc.first },
            if (n.y > acc.second) { n.y } else { acc.second })
    })

    val areaMap = mutableMapOf<Point, Point?>()
    for (x in 0..width) {
        for (y in 0..height) {
            var maxDistance = -1
            var maxCoordinatePoint: Point? = null
            var pointHadEqual: Point? = null
            val mapPoint = Point(x, y)
            for ((coordinatePoint, _) in coordinateMap) {
                val distance = manhattanDistance(coordinatePoint, mapPoint)
                if (distance == maxDistance) {
                    pointHadEqual = coordinatePoint
                }
                maxDistance = if (distance > maxDistance) {
                    maxCoordinatePoint = coordinatePoint
                    distance
                } else {
                    maxDistance
                }
            }
            if (pointHadEqual == maxCoordinatePoint) {
                maxCoordinatePoint = null
            }
            areaMap.put(mapPoint, maxCoordinatePoint)
        }
    }

    return areaMap
    .toString()
}

/** Part 2 */
fun processAreas2(input: String): String {
    return "42"
}

fun manhattanDistance(p1: Int, p2: Int, q1: Int, q2: Int): Int {
    return (p1 - q1).absoluteValue + (p2 - q2).absoluteValue
}

fun manhattanDistance(p1: Point, p2: Point): Int {
    return manhattanDistance(p1.x, p1.y, p2.x, p2.y)
}
