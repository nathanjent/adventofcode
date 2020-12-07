package binary_boarding

import (
	"io/ioutil"
	"testing"
)

const ()

func TestFindHighestSeatIdBasic1(t *testing.T) {
	input := `FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL`
	var want = 820
	result, err := FindHighestSeatId1(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestParseSeatId1(t *testing.T) {
	input := `FBFBBFFRLR`
	var want = 357
	result := ParseSeatId(input)
	if want != result {
		t.Fatalf(`Want %v got %v`, want, result)
	}
}

func TestParseSeatId2(t *testing.T) {
	input := `BFFFBBFRRR`
	var want = 567
	result := ParseSeatId(input)
	if want != result {
		t.Fatalf(`Want %v got %v`, want, result)
	}
}

func TestParseSeatId3(t *testing.T) {
	input := `FFFBBBFRRR`
	var want = 119
	result := ParseSeatId(input)
	if want != result {
		t.Fatalf(`Want %v got %v`, want, result)
	}
}

func TestParseSeatId4(t *testing.T) {
	input := `BBFFBBFRLL`
	var want = 820
	result := ParseSeatId(input)
	if want != result {
		t.Fatalf(`Want %v got %v`, want, result)
	}
}

func TestFindHighestSeatId1(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 987
	result, err := FindHighestSeatId1(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
