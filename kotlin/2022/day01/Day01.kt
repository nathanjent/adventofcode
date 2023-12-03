class Day01 {
  fun part1(input: Iterable<String>): Int {
    return getCaloriesPerElf(input).values.maxOrNull() ?: 0
  }

  fun part2(input: Iterable<String>): Int {
    val caloriesPerElf = getCaloriesPerElf(input)

    // find sum of the top 3 values
    val top3 = mutableListOf<Int>()
    
    for (_i in 1..3) {
      val maxEntry = caloriesPerElf.maxByOrNull { it.value }
      top3.add(maxEntry?.value ?: 0)
      
      // Clear the current max
      caloriesPerElf.remove(maxEntry?.key)
    }
    
    return top3.sum()
  }

  private fun getCaloriesPerElf(input: Iterable<String>) : MutableMap<Int, Int> {
    val caloriesPerElf = mutableMapOf<Int, Int>()
    var cursor = 0;
    
    for (line in input) {
      try {
        val calorieCount = line.toInt()
        val current = caloriesPerElf.getOrDefault(cursor, 0)
        caloriesPerElf[cursor] = calorieCount + current
      } catch (e: NumberFormatException) {
        cursor++
      }
    }

    return caloriesPerElf
  }
}