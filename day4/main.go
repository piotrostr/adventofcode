package main

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func ReadInputFromFile() string {
	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	contents, err := io.ReadAll(f)
	if err != nil {
		panic(err)
	}

	return string(contents)
}

func CheckIfFullyContains(firstRange [2]int, secondRange [2]int) bool {
	// first one contains second
	if firstRange[0] <= secondRange[0] && firstRange[1] >= secondRange[1] {
		return true
	}

	// second one contains first
	if secondRange[0] <= firstRange[0] && secondRange[1] >= firstRange[1] {
		return true
	}

	return false
}

func main() {
	input := ReadInputFromFile()
	pairs := strings.Split(input, "\n")

	fullyContainedRanges := 0

	// drop the last one (empty)
	pairs = pairs[:len(pairs)-1]
	for _, pair := range pairs {
		splitPair := strings.Split(pair, ",")
		first, second := splitPair[0], splitPair[1]

		splitFirstRange := strings.Split(first, "-")
		firstStart, err := strconv.Atoi(splitFirstRange[0])
		if err != nil {
			panic(err)
		}
		firstEnd, err := strconv.Atoi(splitFirstRange[1])
		if err != nil {
			panic(err)
		}
		firstRange := [2]int{
			firstStart,
			firstEnd,
		}

		splitSecondRange := strings.Split(second, "-")
		secondStart, err := strconv.Atoi(splitSecondRange[0])
		if err != nil {
			panic(err)
		}
		secondEnd, err := strconv.Atoi(splitSecondRange[1])
		if err != nil {
			panic(err)
		}
		secondRange := [2]int{
			secondStart,
			secondEnd,
		}
		isFullyContained := CheckIfFullyContains(firstRange, secondRange)
		if isFullyContained {
			fullyContainedRanges += 1
		}
	}

	fmt.Println("fully containted:", fullyContainedRanges)
}
