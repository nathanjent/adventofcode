package report_repair

import (
	"errors"
	"strconv"
	"strings"
)

func FindTwoWithSum2020(input *string) (int64, error) {
	var inputs = strings.Split(*input, "\n")
	var numbers = make([]int64, len(inputs))
	for i, line := range inputs {
		line = strings.Trim(line, "\t ")
		if line == "" {
			continue
		}
		var number, err = strconv.ParseInt(line, 10, 64)
		if err != nil {
			return 0, err
		}
		numbers[i] = number
	}
	for _, i := range numbers {
		for _, j := range numbers {
			if i+j == 2020 {
				return i * j, nil
			}
		}
	}
	return 0, errors.New("not found")
}
