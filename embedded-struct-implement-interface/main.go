package main

type I interface {
	F()
}

type S struct{}

func (S) F() {}

type T struct {
	S
}

func main() {
	var i I = T{}
	i.F()
}
