//go:build cgo

package main

/*
#include "../../link.h"
*/
import "C"

func main() {}

//export Next
func Next(self *C.Field, x, y C.int) C.bool {
	// Count the adjacent cells that are alive.
	alive := 0
	for i := C.int(-1); i <= 1; i++ {
		for j := C.int(-1); j <= 1; j++ {
			if (j != 0 || i != 0) && C.Get(self, x+i, y+j) {
				alive++
			}
		}
	}
	// Return next state according to the game rules:
	//   exactly 3 neighbors: on,
	//   exactly 2 neighbors: maintain current state,
	//   otherwise: off.
	return alive == 3 || alive == 2 && C.Get(self, x, y)
}
