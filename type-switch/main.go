package main

import "fmt"

func main() {
	var i interface{}

	i = 1

	switch i.(type) {
	case int:
		fmt.Println(i.(int) + 1)
	case string:
		fmt.Println(i)
	}
}
