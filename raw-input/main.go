package main

import (
	"fmt"

	termbox "github.com/nsf/termbox-go"
)

func main() {
	must(termbox.Init())
	defer termbox.Close()
	termbox.SetInputMode(termbox.InputAlt | termbox.InputMouse)
	data := make([]byte, 32)
	for {
		switch e := termbox.PollRawEvent(data); e.Type {
		case termbox.EventRaw:
			data := data[:e.N]
			if data[0] == 'q' {
				return
			}
			var e *termbox.Event
		ParseLoop:
			for {
				if e == nil {
					tmpE := termbox.ParseEvent(data)
					e = &tmpE
				}
				switch e.Type {
				case termbox.EventNone:
					e.Type = termbox.EventKey
					e.Key = termbox.KeyEsc
				case termbox.EventKey:
					key := "<none>"
					if e.Ch == 0 {
						switch e.Key {
						case termbox.KeyF1:
							key = "f1"
						case termbox.KeyF2:
							key = "f2"
						case termbox.KeyF3:
							key = "f3"
						case termbox.KeyF4:
							key = "f4"
						case termbox.KeyF5:
							key = "f5"
						case termbox.KeyF6:
							key = "f6"
						case termbox.KeyF7:
							key = "f7"
						case termbox.KeyF8:
							key = "f8"
						case termbox.KeyF9:
							key = "f9"
						case termbox.KeyF10:
							key = "f10"
						case termbox.KeyF11:
							key = "f11"
						case termbox.KeyF12:
							key = "f12"
						case termbox.KeyInsert:
							key = "insert"
						case termbox.KeyDelete:
							key = "delete"
						case termbox.KeyHome:
							key = "home"
						case termbox.KeyEnd:
							key = "end"
						case termbox.KeyPgup:
							key = "pgup"
						case termbox.KeyPgdn:
							key = "pgdown"
						case termbox.KeyArrowUp:
							key = "up"
						case termbox.KeyArrowDown:
							key = "down"
						case termbox.KeyArrowLeft:
							key = "left"
						case termbox.KeyArrowRight:
							key = "right"
						case termbox.KeyCtrlTilde:
							key = "tilde or ctrl+2 or ctrl+space"
						case termbox.KeyCtrlA:
							key = "ctrl+a"
						case termbox.KeyCtrlB:
							key = "ctrl+b"
						case termbox.KeyCtrlC:
							key = "ctrl+c"
						case termbox.KeyCtrlD:
							key = "ctrl+d"
						case termbox.KeyCtrlE:
							key = "ctrl+e"
						case termbox.KeyCtrlF:
							key = "ctrl+f"
						case termbox.KeyCtrlG:
							key = "ctrl+g"
						case termbox.KeyBackspace:
							key = "backspace or ctrl+h"
						case termbox.KeyTab:
							key = "tab or ctrl+i"
						case termbox.KeyCtrlJ:
							key = "ctrl+j"
						case termbox.KeyCtrlK:
							key = "ctrl+k"
						case termbox.KeyCtrlL:
							key = "ctrl+l"
						case termbox.KeyEnter:
							key = "enter or ctrl+m"
						case termbox.KeyCtrlN:
							key = "ctrl+n"
						case termbox.KeyCtrlO:
							key = "ctrl+0"
						case termbox.KeyCtrlP:
							key = "ctrl+p"
						case termbox.KeyCtrlQ:
							key = "ctrl+q"
						case termbox.KeyCtrlR:
							key = "ctrl+r"
						case termbox.KeyCtrlS:
							key = "ctrl+s"
						case termbox.KeyCtrlT:
							key = "ctrl+t"
						case termbox.KeyCtrlU:
							key = "ctrl+u"
						case termbox.KeyCtrlV:
							key = "ctrl+v"
						case termbox.KeyCtrlW:
							key = "ctrl+w"
						case termbox.KeyCtrlX:
							key = "ctrl+x"
						case termbox.KeyCtrlY:
							key = "ctrl+y"
						case termbox.KeyCtrlZ:
							key = "ctrl+z"
						case termbox.KeyEsc:
							key = "esc or ctrl+[ or ctrl+3"
						case termbox.KeyCtrl4:
							key = "ctrl+4 or ctrl+\\"
						case termbox.KeyCtrl5:
							key = "ctrl+5 or ctrl+]"
						case termbox.KeyCtrl6:
							key = "ctrl+6"
						case termbox.KeyCtrl7:
							key = "ctrl+7 or ctrl+/ or ctrl+_"
						case termbox.KeySpace:
							key = "space"
						case termbox.KeyBackspace2:
							key = "backspace2 or ctrl+8"
						}
					}
					mod := "<none>"
					if e.Mod&termbox.ModAlt != 0 {
						mod = "alt"
					}
					fmt.Printf("ch: %q \t key: %s \t mod: %s \t data: %q\n", e.Ch, key, mod, data)
					break ParseLoop
				}
			}
		}

	}
}

func must(err error) {
	if err != nil {
		panic(err)
	}
}
