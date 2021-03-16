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

func TestFindAdaptersInRange(t *testing.T) {
	adapterRatings := []int64{16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4}
	visited := []int64{1, 4, 5, 7, 10}
	var previousJoltage int64 = visited[len(visited)-1]
	want := []int64{11, 12}
	result := FindLargerAdaptersInRange(adapterRatings, previousJoltage)
	if len(want) != len(result) {
		t.Fatalf(`Error: want %v got %v`, want, result)
	}
	for i := 0; i < len(want); i++ {
		if want[i] != result[i] {
			t.Fatalf(`Error: want %v got %v`, want[i], result[i])
		}
	}
}

func TestFindDistinctAdapterArrangmentsBasic1(t *testing.T) {
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
	var want = 8
	result, err := FindDistinctAdapterArrangments(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindDistinctAdapterArrangmentsBasic2(t *testing.T) {
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
	var want = 19208
	result, err := FindDistinctAdapterArrangments(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindDistinctAdapterArrangmentsInputFile(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 396857386627072
	result, err := FindDistinctAdapterArrangments(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
