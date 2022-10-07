package main

import "fmt"

type i int

type s struct {
	x i
	y i
}

func main() {
	m := make(map[s]string)
	m[s{1, 2}] = "hello"
	fmt.Println(m[s{1, 2}])
}
