package main

/*
#cgo CFLAGS: -I..
#cgo LDFLAGS: -L ../multiplier-static/target/release -lmultiplier
#include "multiplier.h"
*/
import "C"
import "fmt"

func main() {
	res := C.multiply(10, -82)
	fmt.Println("GO Static")
	fmt.Println(res)
}
