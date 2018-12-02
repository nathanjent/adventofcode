package aoc.kt.y2018;

/** Part 1 */
fun process1(input: String): String {
    val output = input.lines()
    .filter { !it.isEmpty() }
    .map(Integer::parseInt)
    .sum()

    return output.toString()
}

/** Part 2 */
fun process2(input: String): String {
    val inputs = input.lines()
        .filter { !it.isEmpty() }
        .map(Integer::parseInt)
        .toList()

    val (firstLoop, result) = inputs
        .fold(Pair(mutableListOf<Int>(), 0),
        { (freqs, acc), next ->
            freqs.add(acc)
            Pair(freqs, acc + next)
        })

    var sum = result
    for (_i in 1..999) {
        for (num in inputs) {
            if (firstLoop.contains(sum)) {
                return sum.toString()
            }
            sum += num
        }
    }

    return "It's over 9000!"
}
