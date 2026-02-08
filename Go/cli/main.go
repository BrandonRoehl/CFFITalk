//go:build cgo

package main

/*
#cgo LDFLAGS: -L ../../link/
#cgo LDFLAGS: -l c_get_set -l c_next -l c_step
#include "../../link.h"
*/
import "C"

import (
	"bytes"
	"fmt"
	"math/rand"
	"time"
	"unsafe"
)

type Field struct {
	xyfield []C.bool
	xfield  []*C.bool
	c       C.Field
}

func NewField(w, h int) *Field {
	size := unsafe.Sizeof(C.bool(false))
	f := Field{
		xyfield: make([]C.bool, w*h),
		xfield:  make([]*C.bool, w),

		c: C.Field{
			w: C.int(w),
			h: C.int(h),
		},
	}
	xyfield := unsafe.Pointer(&f.xyfield)
	for x := range w {
		f.xfield[x] = (*C.bool)(unsafe.Add(xyfield, uintptr(x*h)*size))
	}
	return &f
}

func (f *Field) CPtr() *C.Field {
	return (*C.Field)(unsafe.Pointer(&f.c))
}

// String returns the game board as a string.
func (f *Field) String() string {
	var buf bytes.Buffer
	for y := range f.c.h {
		for x := range f.c.w {
			b := byte(' ')
			if C.Get(f.CPtr(), x, y) {
				b = '*'
			}
			buf.WriteByte(b)
		}
		buf.WriteByte('\n')
	}
	return buf.String()
}

type Life struct {
	a *Field
	b *Field
	c C.Life
}

// NewLife returns a new Life game state
func NewLife(w, h int) *Life {
	life := Life{
		a: NewField(w, h),
		b: NewField(w, h),
		c: C.Life{
			w: C.int(w),
			h: C.int(h),
		},
	}
	life.c.a = life.a.CPtr()
	life.c.b = life.b.CPtr()

	return &life
}

// String returns the game board as a string.
func (l *Life) String() string {
	return l.a.String()
}

// CPtr returns a pointer to the C struct for this Life.
func (l *Life) CPtr() *C.Life {
	return (*C.Life)(unsafe.Pointer(&l.c))
}

// Randomize sets a random initial state
func (l *Life) Randomize() {
	for range 50 {
		w := C.int(rand.Intn(int(l.c.w)))
		h := C.int(rand.Intn(int(l.c.h)))
		C.Set(l.c.a, w, h, true)
	}
}

func main() {
	l := NewLife(40, 15)
	fmt.Print("\x1b7")
	for range 300 {
		C.Step(l.CPtr())
		fmt.Print("\x1b8", l) // Reset cursor and print field.
		time.Sleep(time.Second / 3)
	}
}
