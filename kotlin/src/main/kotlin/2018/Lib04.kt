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
    val guardMap = makeGuardMap(input)

    val guardWithMostSleepTime = guardMap.map {
        val guardId = it.key
        val sleepTime = it.value.sumBy { sleepWakeTime ->
            ChronoUnit.MINUTES.between(sleepWakeTime.first, sleepWakeTime.second)
            .toInt()
        }
        Pair(guardId, sleepTime)
    }
    .maxBy { it.second }?.first

    val minuteGuardSleptMost = guardMap.get(guardWithMostSleepTime)?.flatMap {
        val sleepStart = it.first
        val sleepEnd = it.second
        val minutesSlept = mutableMapOf<Int, Long>()
        var timeCursor = sleepStart
        while (timeCursor.until(sleepEnd, ChronoUnit.MINUTES) > 0) {
            val minute = timeCursor.getMinute()
            val minuteCount = minutesSlept.getOrDefault(minute, 0)
            minutesSlept.put(minute, minuteCount + 1)
            timeCursor = timeCursor.plusMinutes(1)
        }
        minutesSlept.toList()
    }?.groupBy {
        it.first
    }?.map {
        Pair(it.key, it.value.sumBy { x -> x.second.toInt() })
    }?.maxBy { it.second }?.first

    val output = guardWithMostSleepTime?.let { a ->
        minuteGuardSleptMost?.let { b -> a * b }
    }

    return output.toString()
}

/** Part 2 */
fun processRepose2(input: String): String {
    val guardMap = makeGuardMap(input)

    val m = guardMap.map {
        val minutesSlept = mutableMapOf<Int, Long>()
        it.value.forEach {
            val sleepStart = it.first
            val sleepEnd = it.second
            var timeCursor = sleepStart
            while (timeCursor.until(sleepEnd, ChronoUnit.MINUTES) > 0) {
                val minute = timeCursor.getMinute()
                val minuteCount = minutesSlept.getOrDefault(minute, 0)
                minutesSlept.put(minute, minuteCount + 1)
                timeCursor = timeCursor.plusMinutes(1)
            }
        }

        Pair(it.key, minutesSlept)
    }
    .map {
        val minuteSleptMost = it.second.maxBy { (_, sleptCount) -> sleptCount }

        Pair(it.first, minuteSleptMost)
    }.map {
        (g, m) -> Triple(g, m?.key, m?.value)
    }.maxBy {
        it.third?:-1
    }

    val output = m?.first?.let { a -> m.second?.let { b -> a * b } }

    return output.toString()
}

private fun makeGuardMap(input: String): MutableMap<Int, MutableList<Pair<LocalDateTime, LocalDateTime>>> {
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
    return guardMap
}
