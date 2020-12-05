package toboggan_trajectory

import (
	"strings"
)

func FindTreeEncounters(input *string) (int, error) {
	downMotion := 1
	rightMotion := 3
	lines := strings.Split(*input, "\n")

	// create a map of trees on the slopes
	mapWidth := len(lines[0])
	mapHeight := len(lines)
	treeMap := make([][]bool, mapHeight)
	for i := range treeMap {
		treeMap[i] = make([]bool, mapWidth)
	}

	// parse tree map from input
	for i, line := range lines {
		for j, char := range line {
			// set if location contains tree
			treeMap[i][j] = char == '#'
		}
	}

	var curPosX, curPosY = 0, 0
	var treeEncounters = 0
	for {
		if curPosY >= mapHeight {
			return treeEncounters, nil
		}

		// map repeats to the right
		mapPosX := curPosX % mapWidth

		locationHasTree := treeMap[curPosY][mapPosX]
		if locationHasTree {
			treeEncounters++
		}

		curPosX += rightMotion
		curPosY += downMotion
	}
}
