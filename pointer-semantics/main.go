package main

type S struct {
	x int
}

func main() {
	s := S{5}
	do(s)
}

func do(s S) {
	s.x = 6
}
