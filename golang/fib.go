package main

import (
	"os"
	"strconv"
)

func fib(i int) int {
	if i < 2 {
		return 1
	}
	return fib(i-1) + fib(i-2)
}

func main() {

	input, _ := strconv.Atoi(os.Args[1])

	fib(input)
}
