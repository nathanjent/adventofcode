package password_philosophy

import (
	"io/ioutil"
	"testing"
)

func TestPasswordPolicyCheckBasic1(t *testing.T) {
	input := `1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc`
	var want = 2
	result, err := PasswordPolicyCheck1(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestPasswordPolicyCheck1(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 477
	result, err := PasswordPolicyCheck1(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestPasswordPolicyCheckBasic2(t *testing.T) {
	input := `1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc`
	var want = 1
	result, err := PasswordPolicyCheck2(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestPasswordPolicyCheck2(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 686
	result, err := PasswordPolicyCheck2(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
