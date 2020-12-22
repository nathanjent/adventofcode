package adapter_array

import (
	"io/ioutil"
	"testing"
)

func TestFindJoltDifferencesBasic1(t *testing.T) {
	input := `16
10
15
5
1
11
7
19
6
12
4`
	var want int64 = 35
	result, err := FindJoltDifferences(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindJoltDifferencesBasic2(t *testing.T) {
	input := `28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3`
	var want int64 = 220
	result, err := FindJoltDifferences(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindJoltDifferencesInputFile(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want int64 = 2263
	result, err := FindJoltDifferences(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
