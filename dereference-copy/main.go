package main

import "fmt"

type s struct {
	x int
}

func main() {
	a := new(s)
	b := *a
	b.x = 5
	fmt.Println(a)
	fmt.Println(b)
}
