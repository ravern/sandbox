package main_test

import "testing"

func BenchmarkMakeBytes(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = make([]byte, 32)
	}
}
