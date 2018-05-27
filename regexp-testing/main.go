package main

import (
	"fmt"
	"regexp"
)

func main() {
	fmt.Println(regexp.MustCompile("^-([a-zA-Z])$").FindStringSubmatch("-d"))
	fmt.Println(regexp.MustCompile("^--([a-zA-Z0-9][a-zA-Z0-9\\-_]*[a-zA-Z0-9])$").FindStringSubmatch("--flag-"))
}
