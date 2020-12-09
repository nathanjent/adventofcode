package toboggan_trajectory

import (
	"io/ioutil"
	"testing"
)

const (
)

func TestFindTreeEncountersBasic1(t *testing.T) {
	input := `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`
	var want = 7
	result, err := FindTreeEncounters(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindTreeEncounters1(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 234
	result, err := FindTreeEncounters(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindTreeEncountersBasic2(t *testing.T) {
	input := `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`
	slopes := []Slope {
		{1,1},
		{3,1},
		{5,1},
		{7,1},
		{1,2},
	}
	var want = 336
	result, err := FindProductOfTreeEncountersForSlopes(&input, slopes)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindTreeEncounters2(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	slopes := []Slope {
		{1,1},
		{3,1},
		{5,1},
		{7,1},
		{1,2},
	}
	var want = 5813773056
	result, err := FindProductOfTreeEncountersForSlopes(&input, slopes)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
