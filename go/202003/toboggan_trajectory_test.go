package toboggan_trajectory

import (
	"io/ioutil"
	"testing"
)

func TestFindTreeEncountersBasic(t *testing.T) {
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

func TestFindTreeEncounters(t *testing.T) {
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
