package handy_haversacks

import (
	//"fmt"
	"strconv"
	"strings"
)

func FindMaxShinyBagContainers(input *string) (int, error) {
	rules, err := ParseRules(input)
	if err != nil {
		return 0, err
	}
	//fmt.Printf("%q\n", rules)

	// find the bag names which could hold my shiny bag
	// if enclosed following the rules
	canContainShinyGoldCount := 0
	var sum int64 = 0
	for containerColor := range rules {
		//fmt.Println()
		// check if container can hold my shiny gold bag
		currentCount, containsShinyBag := CheckRules(0, &containerColor, rules)
		if containsShinyBag {
			canContainShinyGoldCount++
		}
		sum += currentCount
	}

	return canContainShinyGoldCount, nil
}

func CheckRules(sum int64, containerColor *string, rules RuleMap) (int64, bool) {
	containsShinyBag := false
	containedBags := rules[*containerColor]
	//fmt.Printf("%q => %q\n", *containerColor, containedBags)
	if count, ok := containedBags["shiny gold"]; ok {
		// shiny gold found end the search
		//fmt.Println("it can hold a shiny gold bag")
		return sum + count, true
	}

	// The container cannot hold my shiny gold bag
	// directly. Continue searching.
	for bagColor, bagCount := range containedBags {
		if _, ok := rules[bagColor]; ok {
			count, containedShiny := CheckRules(bagCount, &bagColor, rules)
			containsShinyBag = containsShinyBag || containedShiny
			sum += count
		}
	}

	return sum, containsShinyBag
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
