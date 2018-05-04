package main_test

import "testing"

var s = string(make([]rune, 4096))

func BenchmarkStringToRuneArray(b *testing.B) {
	for i := 0; i < b.N; i++ {
		rr := []rune(s)
		for _, _ = range rr {
		}
	}
}

func BenchmarkString(b *testing.B) {
	for i := 0; i < b.N; i++ {
		for _ = range s {
		}
	}
}
