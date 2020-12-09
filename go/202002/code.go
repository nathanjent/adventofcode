package password_philosophy

import (
	"strconv"
	"strings"
)

func PasswordPolicyCheck1(input *string) (int, error) {
	var lines = strings.Split(*input, "\n")

	var validCount = 0
	for _, line := range lines {
		if line == "" { continue }
		entry := strings.SplitN(line, ": ", 2)
		policy := entry[0]
		password := entry[1]
		low, high, requiredLetter, err := ParsePolicy(&policy)
		if err != nil {
			return 0, err
		}
		
		var requiredLetterCount int64 = 0
		for _, c := range password {
			if c == requiredLetter {
				requiredLetterCount++
			}
		}

		if requiredLetterCount <= high && requiredLetterCount >= low {
			validCount++
		}
	}

	return validCount, nil
}

func PasswordPolicyCheck2(input *string) (int, error) {
	var lines = strings.Split(*input, "\n")

	var validCount = 0
	for _, line := range lines {
		if line == "" { continue }
		entry := strings.SplitN(line, ": ", 2)
		policy := entry[0]
		password := entry[1]
		pos1, pos2, requiredLetter, err := ParsePolicy(&policy)
		if err != nil {
			return 0, err
		}

		// positions index starts at 1
		var letterAtPosition1 = []rune(password)[pos1-1]
		var letterAtPosition2 = []rune(password)[pos2-1]
		
		if (letterAtPosition1 == requiredLetter && letterAtPosition2 != requiredLetter) || (letterAtPosition1 != requiredLetter && letterAtPosition2 == requiredLetter) {
			validCount++
		}
	}

	return validCount, nil
}

func ParsePolicy(policy *string) (int64, int64, rune, error) {
	policySet := strings.SplitN(*policy, " ", 2)
	rangeStr := policySet[0]
	requiredLetterStr := policySet[1]
	requiredLetter := []rune(requiredLetterStr)[0]

	ranges := strings.SplitN(rangeStr, "-", 2)
	lowStr := ranges[0]
	highStr := ranges[1]

	low, err := strconv.ParseInt(lowStr, 10, 64)
	if err != nil {
		return 0, 0, requiredLetter, err
	}
	high, err := strconv.ParseInt(highStr, 10, 64)
	if err != nil {
		return low, 0, requiredLetter, err
	}

	return low, high, requiredLetter, nil
}
