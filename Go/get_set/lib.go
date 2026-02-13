//go:build cgo

package main

/*
#include "../../link.h"
*/
import "C"

import "unsafe"

const (
	boolPtrSize = unsafe.Sizeof((*C.bool)(nil))
	boolSize    = unsafe.Sizeof(C.bool(false))
)

func main() {}

func cell(self *C.ConstField, x, y C.int) *C.bool {
	// If the x or y coordinates are outside the field boundaries they are wrapped
	// toroidally. For instance, an x value of -1 is treated as width-1.
	x += self.w
	x %= self.w
	y += self.h
	y %= self.h

	xptr := *(**C.bool)(unsafe.Add(unsafe.Pointer(self.s), uintptr(x)*boolPtrSize))
	xyptr := (*C.bool)(unsafe.Add(unsafe.Pointer(xptr), uintptr(y)*boolSize))

	return xyptr
}

//export Get
func Get(self *C.ConstField, x, y C.int) C.bool {
	return *cell(self, x, y)
}

//export Set
func Set(self *C.ConstField, x, y C.int, b C.bool) {
	*cell(self, x, y) = b
}
