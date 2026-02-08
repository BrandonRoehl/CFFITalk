//go:build cgo

package main

/*
#cgo LDFLAGS: -L ../../link/
#cgo LDFLAGS: -l c_get_set -l c_next -l c_step
#include "../../link.h"
*/
import "C"

import "unsafe"

func newField(w, h int) Field {
	size := unsafe.Sizeof(C.bool(false))
	f := Field{
		xyfield: make([]C.bool, w*h),
		xfield:  make([]*C.bool, w),

		inner: C.Field{
			w: C.int(w),
			h: C.int(h),
		},
	}
	xyfield := unsafe.Pointer(&f.xyfield)
	for x := range w {
		f.xfield[x] = (*C.bool)(unsafe.Add(xyfield, uintptr(x*h)*size))
	}
	return f
}

type Field struct {
	xyfield []C.bool
	xfield  []*C.bool
	inner   C.Field
}

func main() {
	f := newField(10, 10)

	_ = &f.inner
}
