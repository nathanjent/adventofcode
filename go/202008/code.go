package handheld_halting

import (
	"fmt"
	"strconv"
	"strings"
)

func AnalyzeInstructions(input *string) (int, error) {
	program, err := ParseInstructions(input)
	if err != nil {
		return 0, err
	}

	accumulator := 0
	visited := make([]bool, len(program))
	counter := 0
	for ; counter < len(program); {
		fmt.Printf("%v\n", program[counter])
		// stop program if instruction run a second time
		if visited[counter] {
			break;
		}
		visited[counter] = true
		instruction := program[counter]
		switch instruction.Operation {
		case "acc":
			fmt.Printf("acc %v => ", accumulator)
			accumulator += instruction.value
			fmt.Printf("%v\n", accumulator)
			fmt.Printf("pc %v => ", counter)
			counter++
			fmt.Printf("%v\n", counter)
		case "jmp":
			fmt.Printf("jmp %v => ", counter)
			counter += instruction.value
			fmt.Printf("%v\n", counter)
		default:
			// nop
			fmt.Printf("pc %v => ", counter)
			counter++
			fmt.Printf("%v\n", counter)
		}
	}

	return accumulator, nil
}

type Instruction struct {
	Operation string
	value int
}

type Program = []Instruction

// parse rule string into map of containers mapped to
// maps of the contained bag count requirements
func ParseInstructions(input *string) (Program, error) {
	lines := strings.Split(*input, "\n")
	program := Program{}
	for _, line := range lines {
		if line == "" {
			continue
		}

		instructionStrs := strings.Split(line, " ")
		signStr := instructionStrs[1][:1]
		value, err := strconv.ParseInt(instructionStrs[1][1:], 10, 64)
		if err != nil {
			return nil, err
		}
		if signStr == "-" {
			value *= -1
		}
		program = append(program, Instruction{
			instructionStrs[0],
			int(value),
		})
		//fmt.Printf("line=%v, op=%v, val=%v\n", line, instructionStrs[0], value)
	}

	return program, nil
}
