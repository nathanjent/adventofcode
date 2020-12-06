package passport_processing

import (
	"strings"
)

func CountValidPassports(input *string) (int, error) {
	passports := CreatePassports(input)
	var validPassportCount = 0

	for _, passport := range passports {

		if PassportIsValid(passport) {
			validPassportCount++
		}
	}

	return validPassportCount, nil
}

func CreatePassports(input *string) ([]map[string]string) {
	lines := strings.Split(*input, "\n")

	var passports = []map[string]string{}
	passport := map[string]string{}
	passports = append(passports, passport)
	for _, line := range lines {
		if line == "" {
			passport = map[string]string{}
			passports = append(passports, passport)
			continue
		}
		entries := strings.Split(line, " ")
		for _, entry := range entries {
			keyValue := strings.Split(entry, ":")
			key := keyValue[0]
			value := keyValue[1]
			passport[key] = value
		}
	}

	return passports
}

// Validate that all are set except CountryID
// byr BirthYear
// iyr IssueYear
// eyr ExpirationYear
// hgt Height
// hcl HairColor
// ecl EyeColor
// pid PassportID
// cid CountryID
func PassportIsValid(passport map[string]string) (bool) {
	return HasBirthYear(passport) && HasIssueYear(passport) && HasExpirationYear(passport) && HasHeight(passport) && HasHairColor(passport) && HasEyeColor(passport) && HasPassportID(passport)
}

func HasBirthYear(passport map[string]string) (bool) {
	_, ok := passport["byr"]
	return ok
}

func HasIssueYear(passport map[string]string) (bool) {
	_, ok := passport["iyr"]
	return ok
}

func HasExpirationYear(passport map[string]string) (bool) {
	_, ok := passport["eyr"]
	return ok
}

func HasHeight(passport map[string]string) (bool) {
	_, ok := passport["hgt"]
	return ok
}

func HasHairColor(passport map[string]string) (bool) {
	_, ok := passport["hcl"]
	return ok
}

func HasEyeColor(passport map[string]string) (bool) {
	_, ok := passport["ecl"]
	return ok
}

func HasPassportID(passport map[string]string) (bool) {
	_, ok := passport["pid"]
	return ok
}
