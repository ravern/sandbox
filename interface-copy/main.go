package main

import "fmt"

type module interface {
	do()
}

type cloner interface {
	clone() module
}

type s struct {
	x int
}

func (s *s) do() {
	s.x++
}

func (s s) clone() module {
	return &s
}

func main() {
	var a cloner
	a = s{5}

	b := a.clone()

	fmt.Println(a)
	fmt.Println(b)

	b.do()

	fmt.Println(a)
	fmt.Println(b)
}
