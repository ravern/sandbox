package main

import "fmt"

func main() {
	ch := make(chan int, 3)
	ch <- 0
	ch <- 1
	ch <- 2
	close(ch)

	i, ok := <-ch
	fmt.Println(i, ok)
}
