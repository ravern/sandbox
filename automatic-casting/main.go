package main

import "net/http"

type F func(w http.ResponseWriter, r *http.Request)

func A(f F) {
}

func B(w http.ResponseWriter, r *http.Request) {
}

func main() {
	A(B)
}
