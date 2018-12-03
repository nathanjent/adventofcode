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
    var overlapCount = 0
    val claims = input.lines()
        .filter { !it.isEmpty() }
        .map { it.split(" @ ", ": ") }
        .map {
            val idStr = it.get(0)
            val position = it.get(1).split(',')
            val size = it.get(2).split('x')
            Claim(
                    idStr.substring(1 until idStr.length).toInt(),
                    position.get(0).toInt(),
                    position.get(1).toInt(),
                    size.get(0).toInt(),
                    size.get(1).toInt())
        }
        .toList()

    for (claim in claims) {
        for (other in claims) {
            if (claim.id == other.id) {
                continue
            }
            overlapCount += findOverlappingClaims(claim, other))
        }
    }

    return overlapCount.toString()
}

/** Part 2 */
fun process2(input: String): String {
    return "42"
}

fun claimsOverlap(claim1: Claim,claim2: Claim): Int {
    return 1
}
