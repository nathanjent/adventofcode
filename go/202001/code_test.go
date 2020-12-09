package report_repair

import (
	"io/ioutil"
	"testing"
)

func TestFindTwoWithSum2020Basic(t *testing.T) {
	input := `1721
979
366
299
675
1456`
	var want = int64(514579)
	result, err := FindTwoWithSum2020(&input)
	if want != result || err != nil {
		t.Fatalf(`FindTwoWithSum2020() = %q, %v, want match for %v, nil`, input, err, want)
	}
}

func TestFindTwoWithSum2020(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = int64(1014624)
	result, err := FindTwoWithSum2020(&input)
	if want != result || err != nil {
		t.Fatalf(`FindTwoWithSum2020() => %v, %v, want match for %v, nil`, result, err, want)
	}
}

func TestFindThreeWithSum2020Basic(t *testing.T) {
	input := `1721
979
366
299
675
1456`
	var want = int64(241861950)
	result, err := FindThreeWithSum2020(&input)
	if want != result || err != nil {
		t.Fatalf(`FindThreeWithSum2020() = %q, %v, want match for %v, nil`, input, err, want)
	}
}

func TestFindThreeWithSum2020(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = int64(80072256)
	result, err := FindThreeWithSum2020(&input)
	if want != result || err != nil {
		t.Fatalf(`FindThreeWithSum2020() => %v, %v, want match for %v, nil`, result, err, want)
	}
}
