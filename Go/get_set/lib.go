//go:build cgo

package main

/*
#include "../../link.h"
*/
import "C"

import "unsafe"

func main() {}

func cell(self C.ConstFieldPtr, x, y C.int) *C.bool {
	// If the x or y coordinates are outside the field boundaries they are wrapped
	// toroidally. For instance, an x value of -1 is treated as width-1.
	x += self.w
	x %= self.w
	y += self.h
	y %= self.h
	// return unsafe.Add(&unsafe.Add(self.s, x), y)

	size := unsafe.Sizeof(C.bool(false))
	xptr := *(**C.bool)(unsafe.Add(unsafe.Pointer(self.s), uintptr(x)*size))
	xyptr := (*C.bool)(unsafe.Add(unsafe.Pointer(xptr), uintptr(y)*size))

	return xyptr
}

//export Get
func Get(self C.ConstFieldPtr, x, y C.int) C.bool {
	return *cell(self, x, y)
}

//export Set
func Set(self C.ConstFieldPtr, x, y C.int, b C.bool) {
	*cell(self, x, y) = b
}
