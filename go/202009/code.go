package encoding_error

import (
	//"fmt"
	"math"
	"strconv"
	"strings"
)

func FindFirstRuleBreaker(input *string, preambleSize int) (int64, error) {
	numbers, err := ParseNumbers(input)
	if err != nil {
		return 0, err
	}

	return FindFirstRuleBreakerFromNumbers(numbers, preambleSize), nil
}

func FindFirstRuleBreakerFromNumbers(numbers []int64, preambleSize int) int64 {
	var ruleBreaker int64 = -1
	//fmt.Printf("numbers: %v\n", numbers)
	for i := preambleSize; i < len(numbers); i++ {
		number := numbers[i]

		//fmt.Printf("number: %v\n", number)
		min := i - preambleSize
		preamble := numbers[min:i]
		//fmt.Printf("preamble: %v\n", preamble)
		preambleLength := len(preamble)
		pairFound := false
		for i := 0; i < preambleLength; i++ {
			for j := 0; j < preambleLength; j++ {
				a := preamble[i]
				b := preamble[j]

				// two numbers in the pair must be different
				if a == b {
					continue
				}

				sum := a + b
				pairFound = sum == number || pairFound
				if pairFound {
					//fmt.Printf("%v + %v = %v\n", a, b, sum)
				}
			}
		}
		if !pairFound {
			ruleBreaker = number
			break
		}
	}

	return ruleBreaker
}

func FindEncryptionWeakness(input *string, preambleSize int) (int64, error) {
	numbers, err := ParseNumbers(input)
	if err != nil {
		return 0, err
	}

	ruleBreaker := FindFirstRuleBreakerFromNumbers(numbers, preambleSize)

	sequence := FindNumbersWhichSumTo(numbers, ruleBreaker)
	var sum int64 = 0
	if len(sequence) > 1 {
		var max int64 = 0
		var min int64 = math.MaxInt64
		for _, n := range sequence {
			if n > max {
				max = n
			}
			if n < min {
				min = n
			}
		}
		sum = min + max
	}

	return sum, nil
}

func FindNumbersWhichSumTo(numbers []int64, target int64) []int64 {
	sequence := []int64{}
	Out:
	for start := 0; start < len(numbers); start++ {
		for seqLen := 2; seqLen < len(numbers); seqLen++ {
			//fmt.Printf("%v - %v = %v\n", seqLen, start, seqLen - start)
			if seqLen - start > 0 {
				sequence = numbers[start:seqLen]
				//fmt.Printf("sequence: %v\n", sequence)
				var sum int64 = 0
				for _, num := range sequence {
					sum += num
				}
				if sum == target {
					break Out
				}
			}
		}
	}
	return sequence
}

func ParseNumbers(input *string) ([]int64, error) {
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
