package main

import (
	"bufio"
	"os"
	"strconv"
)

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
	println(mostCalories)
}

func main() {
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

	GetMostCaloriesElf(elves)
}
