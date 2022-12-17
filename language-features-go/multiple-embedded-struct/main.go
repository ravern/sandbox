package main

import "fmt"

type three struct {
	one
	two
	three int
}

type one struct {
	one int
}

func (o one) value() int {
	return o.one
}

type two struct {
	two int
}

func (t two) value() int {
	return t.two
}

func main() {
	th := three{
		one: one{
			one: 5,
		},
		two: two{
			two: 10,
		},
	}

	fmt.Println(th.value())
}
