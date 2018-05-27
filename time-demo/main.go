package main

import (
	"fmt"
	"time"
)

func main() {
	inputCh := make(chan string)

	go input(inputCh)
	result(inputCh)
}

func input(inputCh chan string) {
	var input string
	fmt.Scanln(&input)
	inputCh <- input
}

func result(inputCh chan string) {
	timeoutCh := time.After(5 * time.Second)

	select {
	case input := <-inputCh:
		fmt.Printf("success: %s\n", input)
	case <-timeoutCh:
		fmt.Println("failure: timeout")
	}
}
