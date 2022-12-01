package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"strconv"

	"github.com/emirpasic/gods/trees/binaryheap"
	"github.com/emirpasic/gods/utils"
)

var first = flag.Bool("first", false, "first part")

func GetThreeMostCaloriesElvesTotal(elves [][]int) {
	heap := binaryheap.NewWith(func(a, b any) int {
		return -utils.IntComparator(a, b)
	})

	for _, nums := range elves {
		totalCalories := 0
		for _, num := range nums {
			totalCalories += num
		}
		heap.Push(totalCalories)
	}

	total := 0
	for i := 0; i < 3; i++ {
		val, ok := heap.Pop()
		if !ok {
			break
		}
		total += val.(int)
	}

	fmt.Println(total)
}

func GetMostCaloriesElf(elves [][]int) {
	mostCalories := 0
	for _, nums := range elves {
		totalCalories := 0
		for _, num := range nums {
			totalCalories += num
		}
		if totalCalories > mostCalories {
			mostCalories = totalCalories
		}
	}

	fmt.Println(mostCalories)
}

func ParseElvesFromStdin() [][]int {
	scanner := bufio.NewScanner(os.Stdin)
	prev := []byte{}
	elves := [][]int{}
	elf := []int{}
	for {
		scanner.Scan()
		text := scanner.Bytes()

		if len(text) == 0 {
			// no empty elves needed
			if len(elf) != 0 {
				elves = append(elves, elf)
			}
			elf = []int{}
		} else {
			asString, err := strconv.Atoi(string(text))
			if err != nil {
				panic(err)
			}
			elf = append(elf, asString)
		}

		// break on [], [] in a row
		if len(prev) == 0 && len(text) == 0 {
			break
		}
		prev = text
	}

	return elves
}

func main() {
	elves := ParseElvesFromStdin()

	if *first {
		GetMostCaloriesElf(elves)
	} else {
		GetThreeMostCaloriesElvesTotal(elves)
	}
}
