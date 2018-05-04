package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

const name = "Yanundand"

func main() {
	for i := 0; i < 100; i++ {
		var counter int32

		for {
			var (
				gen   []rune
				genMu sync.Mutex
				wg    sync.WaitGroup
			)

			for _, r := range name {
				wg.Add(1)
				go func(r rune) {
					genMu.Lock()
					gen = append(gen, r)
					genMu.Unlock()

					atomic.AddInt32(&counter, 1)

					wg.Done()
				}(r)
			}

			wg.Wait()

			if name == string(gen) {
				break
			}
		}

		fmt.Println("It took", counter, "iterations.")
	}
}
