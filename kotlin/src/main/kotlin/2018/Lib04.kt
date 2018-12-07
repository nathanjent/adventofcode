package aoc.kt.y2018;

import java.time.LocalDateTime
import java.util.Locale
import java.time.format.DateTimeFormatter
import java.time.temporal.ChronoUnit

/**
 * Day 4.
 */

data class Message(val dateTime: LocalDateTime, val text: String)

/** Part 1 */
fun processRepose1(input: String): String {
    val messageMatch = """(\[.*\])|([Gfw].*$)""".toRegex()
    val numMatch = """[0-9]+""".toRegex()
    val dateTimePattern = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm", Locale.ENGLISH)

    var messages = input.lines()
    .filter { !it.isEmpty() }
    .map {
        val (dateTimeStr, message) = messageMatch.findAll(it)
        .map { it.value }
        .toList()

        val dateTime = LocalDateTime.parse(
                dateTimeStr.trim { it == '[' || it == ']' },
                dateTimePattern)
        Message(dateTime, message)
    }
    .sortedBy { it.dateTime }
    .toList()

    // Map of guards to datetime ranges
    val guardMap = mutableMapOf<Int, MutableList<Pair<LocalDateTime, LocalDateTime>>>()

    var currentGuardId = -1
    var asleepStart: LocalDateTime? = null
    for (message in messages) {
        currentGuardId = numMatch.find(message.text)?.value?.toInt()?:currentGuardId

        if (message.text.contains("asleep")) {
            asleepStart = message.dateTime
        }

        if (message.text.contains("wake") && asleepStart != null) {
            val sleepWakeList = guardMap.getOrDefault(currentGuardId, mutableListOf())
            sleepWakeList.add(Pair(asleepStart, message.dateTime))
            guardMap.put(currentGuardId, sleepWakeList)
        }
    }

    val guardWithMostSleepTime = guardMap.map {
        val guardId = it.key
        val sleepTime = it.value.sumBy { sleepWakeTime ->
            ChronoUnit.MINUTES.between(sleepWakeTime.first, sleepWakeTime.second)
            .toInt()
        }
        Pair(guardId, sleepTime)
    }
    .maxBy { it.second }?.first

    //val minuteGuardSleptMost = guardMap.get(guardWithMostSleepTime)
    //    .map {
    //        val sleepStart = it.first
    //        val sleepEnd = it.second
    //        val minutesSlept = mutableMapOf()
    //        for (val time in sleepStart..sleepEnd) {
    //            minutesSlept.put(time.minute, 1)
    //        }
    //        minutesSlept
    //    }

    val output = guardWithMostSleepTime//.map { "\n" + it.toString() }

    return output.toString()
}

/** Part 2 */
fun processRepose2(input: String): String {
    return "42"
}
