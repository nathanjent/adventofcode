package seating_system

import (
	"errors"
	"log"
	"strings"
)

type PositionType int
const (
	Floor PositionType = iota
	Seat
)

type Position struct {
	Type     PositionType
	Occupied bool
}

type SeatingLayout struct {
	Layout [][]Position
}

func FindSeatOccupationAfterStabilization(input *string) (int, error) {
	originalLayout, err := ParseSeatingLayout(input)
	if err != nil {
		return 0, err
	}
	for _, item := range originalLayout.Layout {
		if len(item) == 0 {
			return 0, errors.New("Empty row!!!")
		}
	}

	//log.Println("Finding stable seat occupancy")
	currentLayout := originalLayout
	previousLayout := originalLayout
	// loop until layouts are equal
	for ok := true; ok; ok = !previousLayout.Equals(currentLayout) {
		log.Printf("currentLayout: %v\n", currentLayout.String())
		previousLayout = currentLayout

		currentLayout = ApplyRules(previousLayout)
	}
	//log.Println()
	occupiedCount := currentLayout.Count(func (pos Position) bool {
		return pos.Occupied
	})

	return occupiedCount, nil
}

func ApplyRules(currentLayout SeatingLayout) SeatingLayout {
	newLayout := makeCopy(currentLayout)
	for i, row := range currentLayout.Layout {
		for j, position := range row {
			//log.Printf("[%v, %v] position: %v\n", i, j, position)
			if position.Type == Floor {
				continue
			}

			adjacentOccupiedSeatCount := adjacentCount(i, j, currentLayout.Layout, func (adjacent Position) bool {
				return adjacent.Type == Seat && adjacent.Occupied
			})
			//log.Printf("adjacentOccupiedSeatCount: %v\n", adjacentOccupiedSeatCount)

			// rule 1:
			// empty with no occupied seats adjacent
			// seat becomes occupied
			if !position.Occupied && adjacentOccupiedSeatCount == 0 {
				//log.Println("passenger took a seat")
				newLayout.Layout[i][j].Occupied = true
			}

			// rule 2:
			// occupied with 4+ occupied seats adjacent
			// seat becomes empty
			if position.Occupied && adjacentOccupiedSeatCount >= 4 {
				//log.Println("passenger left a seat")
				newLayout.Layout[i][j].Occupied = false
			}
		}
	}
	return newLayout
}

// return true if predicate is true for all adjacentCount positions in given layout
func adjacentCount(col int, row int, layout [][]Position, predicate func (pos Position) bool) int {
	//log.Printf("[%v, %v]", col, row)
	adjacentCount := 0
	up := col - 1
	down := col + 1
	left := row - 1
	right := row + 1
	// up
	if up >= 0 && predicate(layout[up][row]) {
		//log.Println("up")
		adjacentCount++
	}
	// up right
	if up >= 0 && right < len(layout[up]) && predicate(layout[up][right]) {
		//log.Println("up right")
		adjacentCount++
	}
	// right
	if right < len(layout[col]) && predicate(layout[col][right]) {
		//log.Println("right")
		adjacentCount++
	}
	// down right
	if down < len(layout) && right < len(layout[down]) && predicate(layout[down][right]) {
		//log.Println("down right")
		adjacentCount++
	}
	// down
	if down < len(layout) && predicate(layout[down][row]) {
		//log.Println("down")
		adjacentCount++
	}
	// down left
	if down < len(layout) && left >= 0 && predicate(layout[down][left]) {
		//log.Println("down left")
		adjacentCount++
	}
	// left
	if left >= 0 && predicate(layout[col][left]) {
		//log.Println("left")
		adjacentCount++
	}
	// up left
	if up >= 0 && left >= 0 && predicate(layout[up][left]) {
		//log.Println("up left")
		adjacentCount++
	}

	//log.Printf("adjacentCount: %v\n", adjacentCount)
	return adjacentCount
}

// return count of positions in layout that match predicate
func (seating SeatingLayout) Count(predicate func (pos Position) bool) int {
	count := 0
	for _, row := range seating.Layout {
		for _, pos := range row {
			if predicate(pos) {
				count++
			}
		}
	}
	return count
}

// return true if all positions of this layout are equal to the given layout
func (seating SeatingLayout) Equals(other SeatingLayout) bool {
	if len(seating.Layout) != len(other.Layout) {
		return false
	}
	for i, row := range seating.Layout {
		if len(row) != len(other.Layout[i]) {
			return false
		}
		for j, thisPosition := range row {
			otherPosition := other.Layout[i][j]
			//log.Printf("this: %v == other: %v\n", thisPosition, otherPosition)
			if thisPosition.Type != otherPosition.Type {
				//log.Println("Type not equal.")
				return false
			}
			// check if seat is occupied
			if thisPosition.Type == Seat && thisPosition.Occupied != otherPosition.Occupied {
				//log.Println("Occupied not equal for seat.")
				return false
			}
		}
	}
	//log.Println("Equality true.")
	return true
}

func (seating *SeatingLayout) String() string {
	bldr := strings.Builder{}
	bldr.WriteString("SeatingLayout {\n")
	for _, row := range seating.Layout {
		bldr.WriteRune('\t')
		for _, position := range row {
			switch {
			case position.Type == Floor:
				bldr.WriteRune('.')
			case position.Type == Seat && position.Occupied:
				bldr.WriteRune('#')
			case position.Type == Seat && !position.Occupied:
				bldr.WriteRune('L')
			}
		}
		bldr.WriteRune('\n')
	}
	bldr.WriteString("}\n")
	return bldr.String()
}

// return a deep copy of the given layout
func makeCopy(seating SeatingLayout) SeatingLayout {
	newLayout := make([][]Position, len(seating.Layout))
	for i, row := range seating.Layout {
		newLayout[i] = make([]Position, len(row))
		for j, pos := range row {
			newLayout[i][j] = Position{
				pos.Type,
				pos.Occupied,
			}
		}
	}
	return SeatingLayout{newLayout}
}

// create a layout from the given input string
func ParseSeatingLayout(input *string) (SeatingLayout, error) {
	lines := strings.Split(*input, "\n")
	layout := [][]Position{}
	for _, line := range lines {
		if line == "" {
			continue
		}
		row := make([]Position, len(line))
		layout = append(layout, row)
		for j, r := range line {
			switch r {
			case '.':
				row[j] = Position{Floor, false}
			case 'L':
				row[j] = Position{Seat, false}
			case '#':
				row[j] = Position{Seat, true}
			}
		}
	}

	return SeatingLayout{layout}, nil
}
