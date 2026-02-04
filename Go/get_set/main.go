//go:build cgo

package main

/*
#cgo CFLAGS: -I${SRCDIR}/../../link.h
#cgo LDFLAGS: -linclude
#include "../../link.h"
*/
import "C"

func main() {}

// Execute must match the header file so this import works
//
//export Get
// func Get(self *C.Field, x C.int, y C.int) bool {
// C.
// return true
// }
