package main

/*
#cgo CFLAGS: -I..
#cgo LDFLAGS: -L../multiplier-dynamic/target/release -lmultiplier -L../adder -ladder
#include "multiplier.h"
*/
import "C"
import "fmt"

func main() {
	res := C.multiply(10, -82)
	fmt.Println("GO Dynamic")
	fmt.Println(res)
}
