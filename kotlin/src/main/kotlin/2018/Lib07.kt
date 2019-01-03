package aoc.kt.y2018;

/**
 * Day 7.
 */

data class Node(val id: Char)
data class Edge(val before: Char, val after: Char)

/** Part 1 */
fun processSteps1(input: String): String {
    val stepMatch = " [A-Z] ".toRegex()
    val nodes = mutableSetOf<Node>()
    val edges = mutableSetOf<Edge>()

    input.lines().forEach {
        val (before, after) = stepMatch.findAll(it)
            .map { it.value.trim().toCharArray() }
            .filter { it.size == 1 }
            .map { it.single() }
            .toList()
        nodes.add(Node(before))
        nodes.add(Node(after))
        edges.add(Edge(before, after))
    }

    val output = mutableListOf<Char>()

    // Find order
    var current: Node? = nodes.find { node ->
        !edges.map { it.after }.any { it == node.id }
    }
    if (current != null) {
        output.add(current.id)
        val currentId = current.id

        current = edges.filter { it.before == currentId }
            .sortedBy { it.after }
            .map { Node(it.after) }
            .first()
    }

    return current
        .toString()
}

/** Part 2 */
fun processSteps2(input: String): String {
    return "42"
}
