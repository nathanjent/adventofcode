package adapter_array

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func FindJoltDifferences(input *string) (int64, error) {
	adapterRatings, err := ParseAdapterRatings(input)
	if err != nil {
		return 0, err
	}

	var chargingOutletJoltage int64 = 0
	previousJoltage := chargingOutletJoltage
	checkedAdapters := []int64{}
	var joltageDifferenceOf1Count int64 = 0

	// 3 count starts at 1 to include device adapter rating at 3 higher than any others
	var joltageDifferenceOf3Count int64 = 1
	for i := 0; i < len(adapterRatings); i++ {
		// find adapters within 1-3 difference not yet checked
		adaptersInRange := []int64{}
		for _, adapterRating := range adapterRatings {
			if adapterRating <= previousJoltage + 3 && !any(checkedAdapters, func (n int64) bool {
				return n == adapterRating
			}) {
				adaptersInRange = append(adaptersInRange, adapterRating)
			}
		}
		fmt.Printf("joltagesInRange: %v\n", adaptersInRange)
		var minAdapterRatingInRange int64 = math.MaxInt64
		for _, adapterRating := range adaptersInRange {
			if adapterRating < minAdapterRatingInRange {
				minAdapterRatingInRange = adapterRating
			}
		}
		difference := previousJoltage - minAdapterRatingInRange

		// abs
		if difference < 0 {
			difference = -difference
		}
		if difference == 3.0 {
			joltageDifferenceOf3Count += 1
		}
		if difference == 1.0 {
			joltageDifferenceOf1Count += 1
		}
		previousJoltage = minAdapterRatingInRange
		checkedAdapters = append(checkedAdapters, previousJoltage)
	}

	return joltageDifferenceOf1Count * joltageDifferenceOf3Count, nil
}

func any(items []int64, predicate func(n int64) bool) bool {
	for _, item := range items {
		if predicate(item) {
			return true
		}
	}
	return false
}

func ParseAdapterRatings(input *string) ([]int64, error) {
	lines := strings.Split(*input, "\n")
	numbers := []int64{}
	for _, line := range lines {
		if line == "" {
			continue
		}
		number, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			return numbers, err
		}
		numbers = append(numbers, number)
	}

	return numbers, nil
}
