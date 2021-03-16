package passport_processing

import (
	"errors"
	"strconv"
	"strings"
	"unicode"
)

func CountValidPassports1(input *string) (int, error) {
	passports := CreatePassports(input)
	var validPassportCount = 0
	for _, passport := range passports {
		if PassportHasAllRequiredInfo(passport) {
			validPassportCount++
		}
	}

	return validPassportCount, nil
}

func CountValidPassports2(input *string) (int, error) {
	passports := CreatePassports(input)
	var validPassportCount = 0
	for _, passport := range passports {
		hasValidInfo, err := PassportHasAllValidInfo(passport);
		if err != nil {
			return 0, err
		}
		if  hasValidInfo {
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
func PassportHasAllRequiredInfo(passport map[string]string) (bool) {
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

func PassportHasAllValidInfo(passport map[string]string) (bool, error) {
	hasValidHeight, err := HasValidHeight(passport)
	if err != nil {
		return false, err
	}
	isValid := HasValidBirthYear(passport) && HasValidIssueYear(passport) && HasValidExpirationYear(passport) && hasValidHeight && HasValidHairColor(passport) && HasValidEyeColor(passport) && HasValidPassportID(passport)
	return isValid, nil
}


func HasValidBirthYear(passport map[string]string) (bool) {
	byr, ok := passport["byr"]
	return ok && len(byr) == 4 && byr >= "1920" && byr <= "2002"
}

func HasValidIssueYear(passport map[string]string) (bool) {
	iyr, ok := passport["iyr"]
	return ok && len(iyr) == 4 && iyr >= "2010" && iyr <= "2020"
}

func HasValidExpirationYear(passport map[string]string) (bool) {
	eyr, ok := passport["eyr"]
	return ok && len(eyr) == 4 && eyr >= "2020" && eyr <= "2030"
}

func HasValidHeight(passport map[string]string) (bool, error) {
	if hgt, ok := passport["hgt"]; ok {
		lastIndex := len(hgt)
		if lastIndex < 3 {
			return false, nil
		}
		lastNumberIndex := strings.LastIndexFunc(hgt, unicode.IsNumber) + 1
		heightStr := hgt[0:lastNumberIndex]
		height, err := strconv.ParseInt(heightStr, 10, 64)
		if err != nil {
			return false, err
		}
		if lastNumberIndex >= lastIndex {
			return false, nil
		}

		heightUnits := hgt[lastNumberIndex:lastIndex]
		switch {
		case heightUnits == "cm":
			return height >= 150 && height <= 193, nil
		case heightUnits == "in":
			return height >= 59 && height <= 76, nil
		default:
			return false, errors.New("Invalid height unit: " + heightUnits)
		}
	}

	return false, nil
}

func HasValidHairColor(passport map[string]string) (bool) {
	hcl, ok := passport["hcl"]
	return ok && len(hcl) == 7 && hcl[0] == '#' && strings.ContainsAny(hcl[1:6], "0123456789abcdef")
}

func HasValidEyeColor(passport map[string]string) (bool) {
	ecl, ok := passport["ecl"]
	return ok && len(ecl) == 3 && ecl == "amb" || ecl == "blu" || ecl == "brn" || ecl == "gry" || ecl == "grn" || ecl == "hzl" || ecl == "oth"
}

func HasValidPassportID(passport map[string]string) (bool) {
	pid, ok := passport["pid"]
	return ok && len(pid) == 9 && strings.ContainsAny(pid, "0123456789")
}
