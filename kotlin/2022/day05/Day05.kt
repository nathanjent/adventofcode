class Day05 {

  interface Rule {}

  interface Location: Rule {
    val marker: Char
  }

  data class Empty(
    override val marker: Char = ' '
  ): Location

  data class Crate(
    override val marker: Char
  ): Location

  data class Row(
    val locations: List<Location>
  ): Rule

  data class Move(
    val count: Int,
    val from: Int,
    val to: Int,
  ): Rule

  class Unknown(): Rule

  fun part1(input: Iterable<String>): String {
    val mover = CrateMover()
    mover.loadInstructions(input)

    mover.processMovesWith { move: Move, stacks: MutableMap<Int, ArrayDeque<Location>> ->
      val fromStack = stacks[move.from]
      for (i in 1..move.count) {
        val location = fromStack?.removeFirstOrNull()
        if (location != null && location is Crate) {
          stacks[move.to]?.addFirst(location)
        }
      }
    }

    return mover.stacks.map { it.value.firstOrNull() }
      .filterNotNull()
      .map { it.marker }
      .joinToString("")
  }

  fun part2(input: Iterable<String>): String {
    val mover = CrateMover()
    mover.loadInstructions(input)

    mover.processMovesWith { move: Move, stacks: MutableMap<Int, ArrayDeque<Location>> ->
      val fromStack = stacks[move.from]
      val craneStack: ArrayDeque<Location> = ArrayDeque(listOf())
      for (i in 1..move.count) {
        val location = fromStack?.removeFirstOrNull()
        if (location != null && location is Crate) {
          craneStack.addFirst(location)
        }
      }

      craneStack.forEach {
        stacks[move.to]?.addFirst(it)
      }
    }

    return mover.stacks.map { it.value.firstOrNull() }
      .filterNotNull()
      .map { it.marker }
      .joinToString("")
  }

  class CrateMover {
    private var moveList: List<Move> = listOf()
    val stacks = mutableMapOf<Int, ArrayDeque<Location>>()

    fun loadInstructions(instructions: Iterable<String>) {
      val rules = instructions.map { rule ->
          if (rule.isBlank()) return@map Unknown()
          when (rule[0]) {
            ' ', '[' -> parseRow(rule) ?: Unknown()
            'm' -> parseMove(rule)
            else -> Unknown()
          }
      }
      .filter { it !is Unknown }
      .partition { it is Row }

      generateStacks(rules.first.map { it as Row })
      moveList = rules.second.map { it as Move }
    }

    private fun generateStacks(rows: Iterable<Row>) {
      for (row in rows) {
        row.locations.forEachIndexed {
            i: Int, location: Location ->
          val stackIndex = i + 1 // Stacks are 1-indexed
          val stack = stacks.getOrPut(stackIndex ) {
            ArrayDeque(mutableListOf<Location>())
          }
          when (location) {
            is Crate -> stack.addLast(location)
            is Empty -> {}
            else -> throw Exception("Unknown location type [ $location ]")
          }
        }
      }
    }

    fun processMovesWith(process: (Move, MutableMap<Int, ArrayDeque<Location>>) -> Unit) {
      for (move in moveList) {
        process(move, stacks)
      }
    }

    private fun parseRow(row: String): Row? {
      return Row(row.chunked(4).map {
        when (it[0]) {
          '[' -> {
            when (it[1]) {
              in 'A'..'Z' -> Crate(it[1])
              else -> throw Exception("Unmarked crate error [ $it ]")
            }
          }
          ' ' -> {
            when (it[1]) {
              in '1'..'9' -> return null
              else -> Empty()
            }
          }
          else -> Empty()
        }
      })
    }

    private fun parseMove(move: String): Move {
      val stackMoves = move.split(' ').map {
        when (it) {
          in "1".."9" -> it
          "move", "from", "to" -> null
          else -> throw Exception("Unhandled move index [ $it ]")
        }
      }
        .filterNotNull()

      return Move(
        stackMoves[0].toInt(),
        stackMoves[1].toInt(),
        stackMoves[2].toInt(),
      )
    }
  }
}
