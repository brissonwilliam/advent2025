package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

var VERBOSE = false

func main() {
	fmt.Println("*******************")
	fmt.Println("ADVENT 2025 DAY 1")
	fmt.Println("*******************")

	fmt.Println("-------------------")
	fmt.Println("PART 1")
	fmt.Println("-------------------")

	// Read the input
	filename := "input.txt"
	for _, arg := range os.Args {
		if arg == "--test" {
			fmt.Println("using test.txt")
		}
	}
	fbytes, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	input := string(fbytes)

	// evaluate time for funz. Ignore file io
	startTime := time.Now()

	var pos int16 = 50
	var c int16 = 0
	var czero int16 = 0
	for _, l := range strings.Split(input, "\n") {
		// omit empty lines
		if l == "" {
			fmt.Printf("ignoring empty line at %d\n", c)
			continue
		}
		if len(l) < 2 {
			panic("invalid line input " + l)
		}

		firstChar := l[0]
		offsetStr := l[1:len(l)]
		offset, err := strconv.Atoi(offsetStr)
		if err != nil {
			panic("invalid line input " + l)
		}

		var sign int16
		if firstChar == 'L' || firstChar == 'l' {
			sign = -1
		} else if firstChar == 'R' || firstChar == 'r' {
			sign = 1
		} else {
			panic("invalid line input " + l)
		}

		// instead of simulating every value change
		// be efficient: ignore full spins and just take the remainder of all 100 possible values
		remainder := offset % 100

		// Then juste apply remainder (or small offset in instruction) and re align when out of
		// bounds to  [0..99]
		// At this point we know we can only stay in same range, go pass upper bound 99, pass under bound 0,
		// or land on zero
		var newpos int16 = pos + (sign * int16(remainder))
		if newpos > 99 {
			pos = newpos - 100
		} else if newpos < 0 {
			pos = newpos + 100
		} else {
			pos = newpos
		}
		if pos == 0 {
			czero += 1
		}
		if VERBOSE {
			fmt.Printf("instruction %d | %s | pos %d | total: %d\n", c, l, pos, czero)
		}

		c += 1
	}

	elapsed := time.Since(startTime).String()
	fmt.Printf(
		"finished processing %d lines in %s (without file io)\n",
		c,
		elapsed,
	)
	fmt.Printf("Answer is: %d\n", czero)
}
