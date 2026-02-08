//go:build cgo

package main

/*
#cgo LDFLAGS: -L ../../link/
#cgo LDFLAGS: -l go_get_set -l go_next -l go_step
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
	c       *C.Field
}

func NewField(w, h int) Field {
	size := unsafe.Sizeof(C.bool(false))
	f := Field{
		xyfield: make([]C.bool, w*h),
		xfield:  make([]*C.bool, w),
	}
	// CGo isn't cleaver enough to stop this and will hand it off without
	// realizing it's a Go pointer and not a C pointer
	xyfield := unsafe.Pointer(&f.xyfield[0])
	for x := range w {
		f.xfield[x] = (*C.bool)(unsafe.Add(xyfield, uintptr(x*h)*size))
	}

	f.c = (*C.Field)(C.malloc(C.size_t(unsafe.Sizeof(C.Field{}))))
	f.c.w = C.int(w)
	f.c.h = C.int(h)
	f.c.s = (**C.bool)(unsafe.Pointer(&f.xfield[0]))
	return f
}

// Drop the memory we allocated
func (f *Field) Drop() {
	C.free(unsafe.Pointer(f.c))
}

// String returns the game board as a string.
func (f *Field) String() string {
	var buf bytes.Buffer
	for y := range f.c.h {
		for x := range f.c.w {
			b := byte(' ')
			if C.Get(f.c, x, y) {
				b = '*'
			}
			buf.WriteByte(b)
		}
		buf.WriteByte('\n')
	}
	return buf.String()
}

type Life struct {
	a Field
	b Field
	c *C.Life
}

// NewLife returns a new Life game state
func NewLife(w, h int) *Life {
	life := Life{
		a: NewField(w, h),
		b: NewField(w, h),
		c: &C.Life{
			w: C.int(w),
			h: C.int(h),
		},
	}
	life.c.a = life.a.c
	life.c.b = life.b.c

	return &life
}

// String returns the game board as a string.
func (l *Life) String() string {
	return l.a.String()
}

// Randomize sets a random initial state
func (l *Life) Randomize() {
	for range 50 {
		w := C.int(rand.Intn(int(l.c.w)))
		h := C.int(rand.Intn(int(l.c.h)))
		C.Set(l.c.a, w, h, true)
	}
}

// Drop the memory we allocated
func (l *Life) Drop() {
	l.a.Drop()
	l.b.Drop()
}

func main() {
	fmt.Println("Conway's Game of Life")
	l := NewLife(40, 15)
	defer l.Drop()

	l.Randomize()
	fmt.Print("\x1b7")
	for range 300 {
		C.Step(l.c)
		fmt.Print("\x1b8", l) // Reset cursor and print field.
		time.Sleep(time.Second / 3)
	}
}
