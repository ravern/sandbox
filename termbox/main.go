package main

import (
	"fmt"

	termbox "github.com/nsf/termbox-go"
)

func main() {
	if err := termbox.Init(); err != nil {
		panic(err)
	}
	defer termbox.Close()

	termbox.SetInputMode(termbox.InputEsc | termbox.InputMouse)

	for {
		switch ev := termbox.PollEvent(); ev.Type {
		case termbox.EventKey:
			if ev.Ch == 0 {
				fmt.Println("special key: ", ev.Key)
				break
			}

			fmt.Println("key: ", string(ev.Ch), ev.Mod)

			if ev.Ch == 'q' {
				return
			}

		case termbox.EventMouse:
			fmt.Println("mouse: ", ev.MouseX, ev.MouseY)
		}
	}
}
