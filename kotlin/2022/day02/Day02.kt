class Day02 {
  enum class Result(val score: Int) {
    Win(6),
    Lose(0),
    Draw(3),
  }
  
  enum class Shape(val score: Int) {
    Rock(1),
    Paper(2),
    Scissors(3),
  }
  
  fun part1(input: Iterable<String>): Int {
    return input
      .asSequence()
      .filter { it.isNotBlank() }
      .map { it.split(" ") }
      .map { getRoundPart1(getShape(it[0]), getShape(it[1])) }
      .map { it.first.score + it.second.score }
      .sum()
  }

  fun part2(input: Iterable<String>): Int {
    return input
      .asSequence()
      .filter { it.isNotBlank() }
      .map { it.split(" ") }
      .map { getRoundPart2(getShape(it[0]), getResult(it[1])) }
      .map { it.first.score + it.second.score }
      .sum()
  }
  
  private fun getRoundPart1(opponentShape: Shape, myShape: Shape): Pair<Result, Shape> {
    return when {
      opponentShape == Shape.Rock && myShape == Shape.Paper -> Pair(Result.Win, myShape)
      opponentShape == Shape.Paper && myShape == Shape.Scissors -> Pair(Result.Win, myShape)
      opponentShape == Shape.Scissors && myShape == Shape.Rock -> Pair(Result.Win, myShape)
      
      opponentShape == Shape.Rock && myShape == Shape.Rock -> Pair(Result.Draw, myShape)
      opponentShape == Shape.Paper && myShape == Shape.Paper -> Pair(Result.Draw, myShape)
      opponentShape == Shape.Scissors && myShape == Shape.Scissors -> Pair(Result.Draw, myShape)
      
      else -> Pair(Result.Lose, myShape)
    }
  }

  private fun getRoundPart2(opponentShape: Shape, result: Result): Pair<Result, Shape> {
    return when {
      opponentShape == Shape.Rock && result == Result.Win -> Pair(result, Shape.Paper)
      opponentShape == Shape.Paper && result == Result.Win -> Pair(result, Shape.Scissors)
      opponentShape == Shape.Scissors && result == Result.Win -> Pair(result, Shape.Rock)
      
      opponentShape == Shape.Rock && result == Result.Lose -> Pair(result, Shape.Scissors)
      opponentShape == Shape.Paper && result == Result.Lose -> Pair(result, Shape.Rock)
      opponentShape == Shape.Scissors && result == Result.Lose -> Pair(result, Shape.Paper)
      
      opponentShape == Shape.Rock && result == Result.Draw -> Pair(result, Shape.Rock)
      opponentShape == Shape.Paper && result == Result.Draw -> Pair(result, Shape.Paper)
      opponentShape == Shape.Scissors && result == Result.Draw -> Pair(result, Shape.Scissors)
      
      else -> throw RuntimeException("Unknown round result")
    }
  }

  private fun getShape(shape: String): Shape {
    return when (shape.trim()) {
      "A", "X" -> Shape.Rock
      "B", "Y" -> Shape.Paper
      "C", "Z" -> Shape.Scissors
      else -> throw RuntimeException("Unknown shape input $shape")
    }
  }

  private fun getResult(result: String): Result {
    return when (result.trim()) {
      "X" -> Result.Lose
      "Y" -> Result.Draw
      "Z" -> Result.Win
      else -> throw RuntimeException("Unknown result input $result")
    }
  }
}