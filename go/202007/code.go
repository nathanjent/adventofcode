package handy_haversacks

import (
	"strconv"
	"strings"
)

func FindMaxShinyBagContainers(input *string) (int, error) {
	rules, err := ParseRules(input)
	if err != nil {
		return 0, err
	}

	// find the bag names which could hold my shiny bag
	// if enclosed following the rules
	canContainShinyGoldCount := 0
	for containerColor := range rules {
		// check if container can hold my shiny gold bag
		if CanContainShinyGoldBag(&containerColor, rules) {
			canContainShinyGoldCount++
		}
	}

	return canContainShinyGoldCount, nil
}

func CanContainShinyGoldBag(containerColor *string, rules RuleMap) (bool) {
	containsShinyBag := false
	containedBags := rules[*containerColor]
	if _, ok := containedBags["shiny gold"]; ok {
		// shiny gold found end the search
		return true
	}

	// The container cannot hold my shiny gold bag
	// directly. Continue searching.
	for bagColor, _ := range containedBags {
		containedShiny := CanContainShinyGoldBag(&bagColor, rules)
		containsShinyBag = containsShinyBag || containedShiny
	}

	return containsShinyBag
}

func CountOfBagsRequiredInShinyGoldBag(input *string) (int64, error) {
	rules, err := ParseRules(input)
	if err != nil {
		return 0, err
	}
	bagColor := "shiny gold"
	countOfBags := CountBags(&bagColor, rules)

	return countOfBags, nil
}

func CountBags(containerColor *string, rules RuleMap) (int64) {
	containedBags := rules[*containerColor]
	if len(containedBags) == 0 {
		return 0
	}
	var sum int64 = 0
	for bagColor, bagCount := range containedBags {
		for i := 0; i < int(bagCount); i++ {
			sum += CountBags(&bagColor, rules)
			sum += 1
		}
	}

	return sum
}

type RuleMap = map[string]map[string]int64

// parse rule string into map of containers mapped to
// maps of the contained bag count requirements
func ParseRules(input *string) (RuleMap, error) {
	lines := strings.Split(*input, "\n")
	ruleMap := RuleMap{}
	for _, line := range lines {
		if line == "" {
			continue
		}
		relation := strings.Split(line, " bags contain ")
		containerColor := relation[0]
		containedBagsMap := map[string]int64{}
		ruleMap[containerColor] = containedBagsMap
		containedStr := relation[1]
		containedBagStrs := strings.Split(containedStr, ", ")
		for _, bagStr := range containedBagStrs {
			// trim unnecessary text
			bagStr = strings.TrimSpace(bagStr)
			bagStr = strings.TrimSuffix(bagStr, " bag")
			bagStr = strings.TrimSuffix(bagStr, " bags")
			bagStr = strings.TrimSuffix(bagStr, " bag.")
			bagStr = strings.TrimSuffix(bagStr, " bags.")
			bagCountStrs := strings.SplitN(bagStr, " ", 2)
			bagColor := bagCountStrs[1]
			countStr := bagCountStrs[0]

			// Parse the count. "no" is 0
			if countStr != "no" {
				count, err := strconv.ParseInt(countStr, 10, 64)
				if err != nil {
					return nil, err
				}
				containedBagsMap[bagColor] = count
			}
		}
	}

	return ruleMap, nil
}
