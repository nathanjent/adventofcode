package handy_haversacks

import (
	"io/ioutil"
	"testing"
)

func TestFindMaxShinyBagContainersBasic(t *testing.T) {
	input := `light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.`
	var want = 4
	result, err := FindMaxShinyBagContainers(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestFindMaxShinyBagContainer(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want = 355
	result, err := FindMaxShinyBagContainers(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestCountOfBagsRequiredInShinyGoldBagBasic(t *testing.T) {
	input := `shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.`
	var want int64 = 126
	result, err := CountOfBagsRequiredInShinyGoldBag(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}

func TestCountOfBagsRequiredInShinyGoldBag(t *testing.T) {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		t.Fatalf("Failed to read \"input.txt\". %v", err)
	}
	input := string(data)
	var want int64 = 5312
	result, err := CountOfBagsRequiredInShinyGoldBag(&input)
	if want != result || err != nil {
		t.Fatalf(`Error: "%v", want %v got %v`, err, want, result)
	}
}
