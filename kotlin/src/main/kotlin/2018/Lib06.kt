package aoc.kt.y2018;

import kotlin.math.absoluteValue

/**
 * Day 6.
 */

/** Part 1 */
fun processAreas1(input: String): String {
    val coordinateMap = input.lines()
        .filter { !it.isEmpty() }
        .mapIndexed { i, line ->
            val (xStr, yStr) = line.split(", ")
            Pair(i, Point(xStr.toInt(), yStr.toInt()))
        }
        .toMap()

    val (width, height) = coordinateMap.values.fold(Pair(-1, -1), { acc, n ->
        Pair(if (n.x > acc.first) { n.x } else { acc.first },
        if (n.y > acc.second) { n.y } else { acc.second })
    })

    val distanceMap = mutableMapOf<Point, MutableMap<Int, Int>>()
    for (y in 0..height + 1) {
        for (x in 0..width + 1) {
            val mapPoint = Point(x, y)
            for ((id, coordinatePoint) in coordinateMap) {
                val distance = manhattanDistance(coordinatePoint, mapPoint)
                val distances = distanceMap.getOrDefault(mapPoint, mutableMapOf())
                distances.put(id, distance)
                distanceMap.put(mapPoint, distances)
            }
        }
    }

    val areaMap = mutableMapOf<Point, Int>()
    for ((point, dMap) in distanceMap) {
        val minCoordinate = dMap.minBy { it.value }
        if (minCoordinate != null) {
            val mapWithoutMin = dMap.minus(minCoordinate.key)
            val nextMinCoordinate = mapWithoutMin.minBy { it.value }
            areaMap.put(point, if (nextMinCoordinate == null || minCoordinate.value == nextMinCoordinate.value) {
                -1
            } else {
                minCoordinate.key
            })
        }
    }

    // Remove areas touching border as these are infinite
    val infiniteAreaCoordinates = areaMap.filter {
        val pt = it.key
        pt.x == 0 || pt.y == 0 || pt.x == width || pt.y == height
    }
    .map { it.value }

    val finiteAreaCoordinates = areaMap.filter { point ->
        !infiniteAreaCoordinates.any { infiniteCoordinate ->
            point.value == infiniteCoordinate
        }
    }

    val countMap = mutableMapOf<Int, Int>()
    for ((_, id) in finiteAreaCoordinates) {
        val count = countMap.getOrDefault(id, 0)
        countMap.put(id, count + 1)
    }

    //var visualMap = ""
    //for (y in 0..height + 1) {
    //    for (x in 0..width + 1) {
    //        if (x == 0) {
    //            visualMap += '\n'
    //        }
    //        val mapPoint = Point(x, y)
    //        visualMap += "|" + areaMap.get(mapPoint)?:""
    //    }
    //}

    val maxFiniteAreaSize = countMap.maxBy { it.value }?.value

    return maxFiniteAreaSize
    //visualMap
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
