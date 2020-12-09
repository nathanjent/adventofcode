package custom_customs

import (
	"fmt"
	"strings"
)

func FindSumOfDistinctYes(input *string) (int, error) {
	lines := strings.Split(*input, "\n")
	answerSets := []map[rune]struct{}{}

	// add first group set
	answerSet := map[rune]struct{}{}
	answerSets = append(answerSets, answerSet)

	for _, line := range lines {
		if line == "" {
			// add next group set
			answerSet = map[rune]struct{}{}
			answerSets = append(answerSets, answerSet)
		}

		for _, question := range line {
			// keep a set of distinct yes answers
			answerSet[question] = struct{}{}
		}
	}

	sum := 0
	for _, completedAnswerSet := range answerSets {
		// sum of he count of distinct yes answers
		sum += len(completedAnswerSet)
	}

	return sum, nil
}

func FindSumOfUnanimousYes(input *string) (int, error) {
	lines := strings.Split(*input, "\n")
	answerSets := []map[rune]struct{}{}

	// add first group set
	unanimousAnswerSet := map[rune]struct{}{}
	answerSets = append(answerSets, unanimousAnswerSet)

	answerSetCursor := 0
	answerSetsCursor := 0
	for _, line := range lines {
		if line == "" {
			// add next group set
			unanimousAnswerSet = map[rune]struct{}{}
			answerSets = append(answerSets, unanimousAnswerSet)
			answerSetCursor = 0
			answerSetsCursor++
		}

		if answerSetCursor == 0 {
			// create the initial set of yes answers
			for _, question := range line {
				unanimousAnswerSet[question] = struct{}{}
			}
			continue
		}

		// create a set of yes answers for this line
		thisAnswerSet := map[rune]struct{}{}
		for _, question := range line {
			thisAnswerSet[question] = struct{}{}
		}

		// update running list to remove yes answers that are no longer unanimous
		updatedAnswerSet := map[rune]struct{}{}
		for question := range unanimousAnswerSet {
			if _, ok := unanimousAnswerSet[question]; ok {
				updatedAnswerSet[question] = struct{}{}
			}
		}
		fmt.Printf("line: %v\n", line)
		fmt.Printf("unanimousAnswerSet: %v\n", unanimousAnswerSet)
		fmt.Printf("thisAnswerSet: %v\n", thisAnswerSet)
		fmt.Printf("updatedAnswerSet: %v\n", updatedAnswerSet)

		// replace the running unanimous set
		unanimousAnswerSet = updatedAnswerSet
		answerSets[answerSetsCursor] = unanimousAnswerSet

		answerSetCursor++
	}

	sum := 0
	for _, answerSet := range answerSets {
		// sum of he count of distinct yes answers
		sum += len(answerSet)
	}

	return sum, nil
}
