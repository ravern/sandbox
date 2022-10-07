package main

type I interface {
	F()
}

type S struct {
}

func (s S) F() {

}

func main() {
	var i I
	i = S{}
}
