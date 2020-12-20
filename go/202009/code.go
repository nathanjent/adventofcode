package encoding_error

import (
	"fmt"
	"strconv"
	"strings"
)

func FindFirstRuleBreaker(input *string, preambleSize int) (int64, error) {
	numbers, err := ParseNumbers(input)
	if err != nil {
		return 0, err
	}
	var ruleBreaker int64 = -1
	fmt.Printf("numbers: %v\n", numbers)
	for i := preambleSize; i < len(numbers); i++ {
		number := numbers[i]

		fmt.Printf("number: %v\n", number)
		min := i - preambleSize
		preamble := numbers[min:i]
		fmt.Printf("preamble: %v\n", preamble)
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
			fmt.Printf("Rulebreaker = %v\n", number)
			ruleBreaker = number
			break
		}
	}

	return ruleBreaker, nil
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
