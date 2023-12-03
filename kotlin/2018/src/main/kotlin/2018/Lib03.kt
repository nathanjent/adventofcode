package aoc.kt.y2018;

/**
 * Day 3.
 */

data class Point(
        val x: Int,
        val y: Int)

/** Part 1 */
fun processClaims1(input: String): String {
    val claimMap = getClaimMap(input)

    val overlapCount = claimMap.filter { it.value.count() > 1 } .count()

    return overlapCount.toString()
}

/** Part 2 */
fun processClaims2(input: String): String {
    val claimMap = getClaimMap(input)

    val claimsIntact = mutableMapOf<Int, Boolean>()
    val id = claimMap
        .map { Pair(it.key, it.value.toList()) }
        .fold(claimsIntact, { acc, next ->
            val idList = next.second
            if (idList.count() > 1) {
                idList.forEach { acc.put(it, false) }
            } else {
                val id = idList.first()
                val intact = acc.getOrDefault(id, true)
                if (intact) {
                    acc.put(id, true)
                }
            }
            acc
        })
        .filter { it.value }
        .map { it.key }
        .first()

    return id.toString()
}

fun getClaimMap(input: String): MutableMap<Point, MutableList<Int>> {
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
    return claimMap
}
