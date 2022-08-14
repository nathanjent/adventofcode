package encoding_error

import (
	"io/ioutil"
	"strconv"
	"strings"
	"testing"
)

func TestFindFirstRuleBreakerBasic1(t *testing.T) {
	lines := make([]string, 25)
	for i := 0; i < len(lines); i++ {
		lines[i] = strconv.FormatInt(int64(i+1), 10)
	}
	lines = append(lines, "50")
	input := strings.Join(lines, "\n")
	var want int64 = 50
	result, err := FindFirstRuleBreaker(&input, 25)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

const (
	basicInput = `35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576`
)

func TestFindFirstRuleBreakerBasic2(t *testing.T) {
	input := basicInput
	var want int64 = 127
	result, err := FindFirstRuleBreaker(&input, 5)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindFirstRuleBreakerInputFile(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want int64 = 36845998
	result, err := FindFirstRuleBreaker(&input, 25)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestEncryptionWeaknessBasic1(t *testing.T) {
	input := basicInput
	var want int64 = 62
	result, err := FindEncryptionWeakness(&input, 5)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestEncryptionWeaknessFileInput(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want int64 = 4830226
	result, err := FindEncryptionWeakness(&input, 25)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
