package main

import "fmt"

type runeSlice []rune

func newRuneSlice(rr []rune) *runeSlice {
	rs := runeSlice(rr)
	return &rs
}

func (rr *runeSlice) change() {
	*rr = append(*rr, 'a')
}

func main() {
	rr := newRuneSlice([]rune("nibb"))
	rr.change()
	fmt.Printf("%q\n", rr)
}
