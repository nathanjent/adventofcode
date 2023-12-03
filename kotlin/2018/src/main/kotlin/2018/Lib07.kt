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


    // Find first step
    var current: Node? = nodes.find { node ->
        !edges.map { it.after }.any { it == node.id }
    }

    // Find remaining steps
    val visited = mutableListOf<Char>()
    while (current != null) {
        val currentId = current.id
        visited.add(currentId)
        val nextList = edges
            .filter { it.before == currentId }
            .sortedBy { it.after }
            .map { it.after }

        if (!nextList.isEmpty()) {
            current = nodes.find { it.id == nextList.first() }
        } else {
            current = null
        }
    }

    return visited
        .toString()
}

/** Part 2 */
fun processSteps2(input: String): String {
    return "42"
}
