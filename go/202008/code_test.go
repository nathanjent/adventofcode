package handheld_halting

import (
	"io/ioutil"
	"testing"
)

func TestCheckProgramBasic(t *testing.T) {
	input := `nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6`
	var want = 5
	result, err := CheckProgramForInfiniteLoop(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestCheckProgram(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 1521
	result, err := CheckProgramForInfiniteLoop(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestAutoCorrectingProgramExecBasic(t *testing.T) {
	input := `nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6`
	var want = 8
	result, err := AutoCorrectingProgramExec(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestAutoCorrectingProgramExec(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 1016
	result, err := AutoCorrectingProgramExec(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
