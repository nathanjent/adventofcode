class Day06 {

  fun part1(input: String): Int {
    return findStartOfPacket(input, 4)
  }

  fun part2(input: String): Int {
    return findStartOfPacket(input, 14)
  }

  private fun findStartOfPacket(input: String, packetMarkerLength: Int): Int {
    var cursor = 0
    while (cursor + packetMarkerLength < input.length) {
      val window = input.subSequence(cursor, cursor + packetMarkerLength)
      if (isMarker(window)) break
      cursor++
    }

    return cursor + packetMarkerLength
  }

  private fun isMarker(markerStr: CharSequence): Boolean {
    markerStr.asIterable().forEachIndexed { i, c ->
      markerStr.asIterable().forEachIndexed { j, d ->
        if (i != j) {
          if (c == d) {
            return false
          }
        }
      }
    }

    return true
  }
}
