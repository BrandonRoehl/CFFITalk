//go:build cgo

package main

/*
#include "../../link.h"
*/
import "C"

func main() {}

//export Step
func Step(self *C.Life) {
	// Update the state of the next field (b) from the current field (a).
	for y := C.int(0); y < self.h; y++ {
		for x := C.int(0); x < self.w; x++ {
			C.Set(self.b, x, y, C.Next(self.a, x, y))
		}
	}
	// Swap fields a and b.
	self.a, self.b = self.b, self.a
}
