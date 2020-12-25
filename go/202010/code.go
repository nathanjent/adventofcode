package adapter_array

import (
	"log"
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
		adaptersInRange := FindLargerAdaptersInRange(adapterRatings, previousJoltage)
		//log.Printf("adaptersInRange: %v\n", adaptersInRange)
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

func FindDistinctAdapterArrangments(input *string) (int, error) {
	adapterRatings, err := ParseAdapterRatings(input)
	if err != nil {
		return 0, err
	}

	var chargingOutletJoltage int64 = 0
	//visitedAdapters := []int64{}
	counter := Counter{}
	mem := MemoizedCount{}
	FindAdapterArrangements(adapterRatings, chargingOutletJoltage, &counter, &mem /*, visitedAdapters*/)
	log.Printf("memoized counts: %v\n", mem)

	return counter.Count, nil
}

type MemoizedCount = map[int64]int

func FindAdapterArrangements(adapterRatings []int64, previousJoltage int64, counter *Counter, mem *MemoizedCount /*, visited []int64*/) {
	//log.Printf("previousJoltage: %v\n", previousJoltage)
	adaptersInRange := FindLargerAdaptersInRange(adapterRatings, previousJoltage)
	if len(adaptersInRange) == 0 {
		counter.Increment()
		return
	}
	for _, adapterRating := range adaptersInRange {
		//visitedBranch := makeCopy(visited)
		//visitedBranch = append(visitedBranch, adapterRating)
		memoized := *mem
		if memCount, ok := memoized[adapterRating]; ok {
			counter.Add(memCount)
		} else {
			branchCounter := Counter{}
			FindAdapterArrangements(adapterRatings, adapterRating, &branchCounter, mem /*, visitedBranch*/)
			counter.Add(branchCounter.Count)
			memoized[adapterRating] = branchCounter.Count
		}
	}
}

type Counter struct{ Count int }

func (counter *Counter) Increment() { counter.Count++ }
func (counter *Counter) Add(count int) {
	counter.Count += count
}

// find adapters within 1-3 difference but not in the filtered slice
func FindLargerAdaptersInRange(adapterRatings []int64, joltage int64) []int64 {
	adaptersInRange := []int64{}
	for _, adapterRating := range adapterRatings {
		if adapterRating > joltage && adapterRating <= joltage+3 {
			adaptersInRange = append(adaptersInRange, adapterRating)
		}
	}

	return adaptersInRange
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
