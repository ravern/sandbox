package main

import (
	"fmt"
	"time"

	"github.com/ivoeditor/core"
	"github.com/ivoeditor/graphics"
	"github.com/ivoeditor/loop"
	"github.com/ivoeditor/termbox"
)

type module struct {
	rendered bool
	cols     int
	rows     int
}

func (m *module) Send(e core.Event) {
	if e, ok := e.(graphics.Resize); ok {
		m.cols = e.Cols
		m.rows = e.Rows
		return
	}
	fmt.Println(e)
}

func (m *module) Recv() core.Event {
	if !m.rendered {
		m.rendered = true
		for m.cols == 0 || m.rows == 0 {
		}
		cells := graphics.NewCells(m.cols, m.rows)
		return graphics.Render{
			Cells: cells,
		}
	}
	time.Sleep(1 * time.Second)
	return loop.Close{}
}

func main() {
	module := &module{}
	termbox := &termbox.Module{
		Module: module,
	}
	termbox.Send(loop.Init{})
	fmt.Println(termbox.Recv())
}
