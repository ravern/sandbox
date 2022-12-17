package main

func main() {
	a := 1
	b := 1
	switch a, b {
	case 1, 2:
		fmt.Println("1, 2")
	default:
		fmt.Println("not found")
	}
}
