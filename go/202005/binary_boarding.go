package binary_boarding

import (
	"strings"
)

func FindHighestSeatId1(input *string) (int, error) {
	seatIds := ParseSeatIds(input)
	var maxSeatId = 0
	for _, seatId := range seatIds {
		if maxSeatId < seatId {
			maxSeatId = seatId
		}
	}

	return maxSeatId, nil
}

func ParseSeatIds(input *string) []int {
	lines := strings.Split(*input, "\n")
	seatIds := []int{}
	for _, line := range lines {
		if line == "" {
			continue
		}
		seatId := ParseSeatId(line)
		seatIds = append(seatIds, seatId)
	}

	return seatIds
}

// Parse string to calculate seat id
func ParseSeatId(input string) int {
	row := ParseRow(input[0:7])
	column := ParseColumn(input[7:])
	seatId := row*8 + column
	return seatId
}

// Parses input as binary mapping to row value
// 'B' means to keep the upper half
// 'F' means to keep the lower half
func ParseRow(input string) int {
	hi := 127
	lo := 0
	for _, r := range input {
		if r == 'F' {
			hi -= (hi-lo)/2 + 1
		} else {
			lo += (hi-lo)/2 + 1
		}
	}
	return hi
}

// Parses input as binary mapping to column value
// 'R' means to keep the upper half
// 'L' means to keep the lower half
func ParseColumn(input string) int {
	hi := 7
	lo := 0
	for _, r := range input {
		if r == 'L' {
			hi -= (hi-lo)/2 + 1
		} else {
			lo += (hi-lo)/2 + 1
		}
	}
	return hi
}
