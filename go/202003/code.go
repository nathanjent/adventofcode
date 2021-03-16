package toboggan_trajectory

import (
	"strings"
)

type TreeMap struct {
	treeLocations [][]bool
	mapWidth int
	mapHeight int
}

type Slope struct {
	RightMotion int
	DownMotion int
}

func FindTreeEncounters(input *string) (int, error) {
	treeMap := CreateTreeMap(input)
	slope := Slope { 3, 1 }
	treeEncounters := FindTreeEncountersForSlope(treeMap, slope)
	return treeEncounters, nil
}

func FindProductOfTreeEncountersForSlopes(input *string, slopes []Slope) (int, error) {
	treeMap := CreateTreeMap(input)
	var treeEncountersProduct = 1
	for _, slope := range slopes {
		treeEncountersProduct *= FindTreeEncountersForSlope(treeMap, slope)
	}
	return treeEncountersProduct, nil
}

func FindTreeEncountersForSlope(treeMap TreeMap, slope Slope) (int) {
	var curPosX, curPosY = 0, 0
	var treeEncounters = 0
	for {
		if curPosY >= treeMap.mapHeight {
			return treeEncounters
		}

		mapPosX := curPosX % treeMap.mapWidth
		locationHasTree := treeMap.treeLocations[curPosY][mapPosX]
		if locationHasTree {
			treeEncounters++
		}

		curPosX += slope.RightMotion
		curPosY += slope.DownMotion
	}
}

func CreateTreeMap(input *string) (TreeMap) {
	lines := strings.Split(*input, "\n")

	mapWidth := len(lines[0])
	mapHeight := len(lines)
	treeLocations := make([][]bool, mapHeight)
	for i := range treeLocations {
		treeLocations[i] = make([]bool, mapWidth)
	}

	for i, line := range lines {
		for j, char := range line {

			treeLocations[i][j] = char == '#'
		}
	}

	return TreeMap{ treeLocations, mapWidth, mapHeight }
}
