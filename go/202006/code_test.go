package custom_customs

import (
	"io/ioutil"
	"testing"
)

func TestFindSumOfDistinctYesBasic(t *testing.T) {
	input := `abc

a
b
c

ab
ac

a
a
a
a

b`
	var want = 11
	result, err := FindSumOfDistinctYes(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindSumOfDistinctYes(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 6625
	result, err := FindSumOfDistinctYes(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindSumOfUnanimousYesBasic(t *testing.T) {
	input := `abc

a
b
c

ab
ac

a
a
a
a

b`
	var want = 6
	result, err := FindSumOfUnanimousYes(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindSumOfUnanimousYes(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 3360
	result, err := FindSumOfUnanimousYes(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
