package seating_system

import (
	"io/ioutil"
	"log"
	"testing"
)

func TestLayoutsEqual(t *testing.T) {
	layout1 := make([][]Position, 1)
	layout1[0] = []Position{
		{ Seat, true },
		{ Floor, true },
	}
	layout2 := make([][]Position, 1)
	layout2[0] = []Position{
		{ Seat, true },
		{ Floor, true },
	}
	a := SeatingLayout{ layout1 }
	b := SeatingLayout{ layout2 }
	want := true
	log.Printf("%v\n", a.String())
	log.Printf("%v\n", b.String())
	result := a.Equals(b)
	if want != result {
		t.Fatalf(`want %v got %v`, want, result)
	}
}

func TestLayoutsNotEqual1(t *testing.T) {
	layout1 := make([][]Position, 1)
	layout1[0] = []Position{
		{ Seat, true },
		{ Floor, false },
	}
	layout2 := make([][]Position, 1)
	layout2[0] = []Position{
		{ Seat, true },
		{ Seat, true },
	}
	a := SeatingLayout{ layout1 }
	b := SeatingLayout{ layout2 }
	want := false
	log.Printf("%v", a.String())
	log.Printf("%v", b.String())
	result := a.Equals(b)
	if want != result {
		t.Fatalf(`want %v got %v`, want, result)
	}
}

func TestLayoutsNotEqual2(t *testing.T) {
	layout1 := make([][]Position, 2)
	// #.LL.L#.##
	layout1[0] = []Position{
		{ Seat, true },
		{ Floor, false },
		{ Seat, false },
		{ Seat, false },
		{ Floor, false },
		{ Seat, false },
		{ Seat, true },
		{ Floor, false },
		{ Seat, false },
		{ Seat, false },
	}
	// #LLLLLL.L#
	layout1[1] = []Position{
		{ Seat, true },
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
		{ Floor, false },
		{ Seat, false },
		{ Seat, true },
	}
	layout2 := make([][]Position, 2)
	// #.##.##.##
	layout2[0] = []Position{
		{ Seat, true },
		{ Floor, false },
		{ Seat, true },
		{ Seat, true },
		{ Floor, false },
		{ Seat, true },
		{ Seat, true },
		{ Floor, false },
		{ Seat, true },
		{ Seat, true },
	}
	// #LLLLL#.##
	layout2[1] = []Position{
		{ Seat, true },
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
		{ Seat, true },
		{ Floor, false },
		{ Seat, true },
		{ Seat, true },
	}
	a := SeatingLayout{ layout1 }
	b := SeatingLayout{ layout2 }
	want := false
	log.Printf("%v", a.String())
	log.Printf("%v", b.String())
	result := a.Equals(b)
	if want != result {
		t.Fatalf(`want %v got %v`, want, result)
	}
}

func TestAdajentCount(t *testing.T) {
	layout := make([][]Position, 3)
	layout[0] = []Position{
		{ Seat, false },
		{ Seat, false },
		{ Seat, false },
	}
	layout[1] = []Position{
		{ Seat, false },
		{ Seat, false },
		{ Seat, true },
	}
	layout[2] = []Position{
		{ Seat, true },
		{ Seat, false },
		{ Seat, false },
	}
	want := 6
	result := adjacentCount(1, 1, layout, func(pos Position) bool {
		return pos.Type == Seat && !pos.Occupied
	})
	if want != result {
		t.Fatalf(`want %v got %v`, want, result)
	}
}

func TestApplyRules(t *testing.T) {
	input := `#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##`
	want := `#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##`
	currentLayout, err := ParseSeatingLayout(&input)
	if err != nil {
		t.Fatalf(`Parse error: "%v"`, err)
	}
	wantedLayout, err := ParseSeatingLayout(&want)
	if err != nil {
		t.Fatalf(`Parse error: "%v"`, err)
	}
	result := ApplyRules(currentLayout)
	log.Printf("%v\n", currentLayout.String())
	log.Printf("%v\n", result.String())
	if !wantedLayout.Equals(result) {
		t.Fatalf(`want %v got %v`, wantedLayout.String(), result.String())
	}
}

func TestFindSeatOccupationBasic1(t *testing.T) {
	input := `L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL`
	want := 37
	result, err := FindSeatOccupationAfterStabilization(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindSeatOccupationInputFile(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	want := 2324
	result, err := FindSeatOccupationAfterStabilization(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
