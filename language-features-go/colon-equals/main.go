package main

import "fmt"

func main() {
	i := 0
	func() {
		var j int
		i, j = test()
		fmt.Println(i, j)
	}()
	fmt.Println(i)
}

func test() (int, int) {
	return 2, 3
}
