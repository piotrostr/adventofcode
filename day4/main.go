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

func CheckIfFullyContains(a [2]int, b [2]int) bool {
	// first one contains second
	if a[0] <= b[0] && a[1] >= b[1] {
		return true
	}

	// second one contains first
	if b[0] <= a[0] && b[1] >= a[1] {
		return true
	}

	return false
}

func CheckIfOverlapping(a [2]int, b [2]int) bool {

	// the containing cases
	if CheckIfFullyContains(a, b) {
		return true
	}

	// left overlaps right from left side
	if a[0] <= b[0] && b[0] <= a[1] && b[1] >= a[1] {
		return true
	}

	// left overlaps right from right side
	if b[0] <= a[0] && a[0] <= b[1] && a[1] >= b[1] {
		return true
	}

	return false
}

func main() {
	input := ReadInputFromFile()
	pairs := strings.Split(input, "\n")

	fullyContainedRanges := 0
	overlappingRanges := 0

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

		isOverlapping := CheckIfOverlapping(firstRange, secondRange)
		if isOverlapping {
			overlappingRanges += 1
		}
	}

	fmt.Println("fully contained:", fullyContainedRanges)
	fmt.Println("overlapping:", overlappingRanges)
}
