package aoc.kt.y2018;

/**
 * Day 2.
 */

/** Part 1 */
fun processBoxChecksum1(input: String): String {
    val (count2s, count3s) = input.lines()
        .filter { !it.isEmpty() }
        .map {
            val charCounts = mutableMapOf<Char, Int>()
            it.forEach { c ->
                val count = charCounts.getOrDefault(c, 0)
                charCounts.put(c, count + 1)
            }
            Pair(if (charCounts.containsValue(2)) 1 else 0,
                if (charCounts.containsValue(3)) 1 else 0)
        }
        .fold(Pair(0, 0), {
            (count2s, count3s), (two, three) ->
            Pair(count2s + two, count3s + three)
        })

    return (count2s * count3s).toString()
}

/** Part 2 */
fun processBoxChecksum2(input: String): String {
    val lines = input.lines()
        .filter { !it.isEmpty() }
    val matchCounts = mutableMapOf<String, Pair<Int, Int>>()

    for (l1 in lines) {
        for (l2 in lines) {
            if (l1 == l2) continue
            // use combined lines as key
            val key = "$l1:$l2"
            l1.zip(l2).forEachIndexed({
                index, (c1, c2) ->
                val (count, pIndex) =
                    matchCounts.getOrDefault(key, Pair(0, -1))
                if (c1 == c2) {
                    // count matching characters
                    matchCounts.put(key, Pair(count + 1, pIndex))
                } else {
                    // index of the non-matching character
                    matchCounts.put(key, Pair(count, index))
                }
            })
        }
    }

    val maxEntry = matchCounts
        .maxBy { (_, p) -> p.first }

    val (_, index) = maxEntry?.component2()?:Pair(0, -1)
    val key = maxEntry?.component1()?:""
    val word = key.substringBefore(':')
    val first = word.substring(0 until index)
    val last = word.substring(index+1..word.length-1)

    return "$first$last"
}
