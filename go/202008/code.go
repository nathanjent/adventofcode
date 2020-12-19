package handheld_halting

import (
	"errors"
	//"fmt"
	"strconv"
	"strings"
)

func CheckProgramForInfiniteLoop(input *string) (int, error) {
	program, err := ParseInstructions(input)
	if err != nil {
		return 0, err
	}

	accumulator, err := RunProgram(program)
	if !errors.Is(err, InfiniteLoopError) {
		return 0, err
	}

	return accumulator, nil
}

func AutoCorrectingProgramExec(input *string) (int, error) {
	program, err := ParseInstructions(input)
	if err != nil {
		return 0, err
	}

	accumulator, err := 0, nil
	for cursor := 0; cursor < len(program); cursor++ {
		// Attempt to correct current instruction
		program[cursor].Operation = FlipOperation(program[cursor].Operation)
		//fmt.Printf("%v\n", program)
		accumulator, err = RunProgram(program)
		if err == nil || !errors.Is(err, InfiniteLoopError) {
			// Some other error occurred
			break
		}

		// Revert the program modification
		program[cursor].Operation = FlipOperation(program[cursor].Operation)
	}

	return accumulator, err
}

// Returns a NOP for a JMP or a JMP for a NOP instruction
func FlipOperation(operation string) string {
	switch operation {
	case "nop":
		return "jmp"
	case "jmp":
		return "nop"
	default:
		return operation
	}
}

func RunProgram(program Program) (int, error) {
	//fmt.Printf("\n%v\n", program)
	accumulator := 0
	visited := make([]bool, len(program))
	counter := 0
	for counter < len(program) {
		//fmt.Printf("%v\n", program[counter])
		// stop program if instruction run a second time
		if visited[counter] {
			return accumulator, InfiniteLoopError
		}
		visited[counter] = true
		instruction := program[counter]
		switch instruction.Operation {
		case "acc":
			//fmt.Printf("acc %v => ", accumulator)
			accumulator += instruction.Value
			//fmt.Printf("%v\n", accumulator)
			//fmt.Printf("pc %v => ", counter)
			counter++
			//fmt.Printf("%v\n", counter)
		case "jmp":
			//fmt.Printf("jmp %v => ", counter)
			counter += instruction.Value
			//fmt.Printf("%v\n", counter)
		default:
			// nop
			//fmt.Printf("pc %v => ", counter)
			counter++
			//fmt.Printf("%v\n", counter)
		}
	}

	return accumulator, nil
}

var InfiniteLoopError = errors.New("Infinite loop detected.")

type Instruction struct {
	Operation string
	Value     int
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
