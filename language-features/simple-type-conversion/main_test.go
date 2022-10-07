package main_test

import "testing"

type custom int

func BenchmarkNormalAdd(b *testing.B) {
	aa := 1
	bb := 2
	for i := 0; i < b.N; i++ {
		_ = aa + bb
	}
}

func BenchmarkOneCustomAdd(b *testing.B) {
	aa := custom(1)
	bb := 2
	for i := 0; i < b.N; i++ {
		_ = int(aa) + bb
	}
}

func BenchmarkBothCustomAdd(b *testing.B) {
	aa := custom(1)
	bb := custom(2)
	for i := 0; i < b.N; i++ {
		_ = int(aa) + int(bb)
	}
}
