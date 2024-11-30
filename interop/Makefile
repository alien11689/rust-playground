all: static dynamic 

static:
	make -C adder clean
	make -C adder static
	cd multiplier-static && cargo clean && cargo test && cargo build --release
	rm -f demo-static
	gcc -o demo-static main.c -L multiplier-static/target/release -lmultiplier
	./demo-static
	cd go-static && go run . && cd -

dynamic:
	make -C adder clean
	make -C adder dynamic
	cd multiplier-dynamic && cargo clean && LD_LIBRARY_PATH="`pwd`/../adder" cargo test && cargo build --release
	rm -f demo-dynamic
	LD_LIBRARY_PATH=adder gcc -o demo-dynamic main.c -lmultiplier -L multiplier-dynamic/target/release
	LD_LIBRARY_PATH=multiplier-dynamic/target/release:adder ./demo-dynamic
	cd go-dynamic && LD_LIBRARY_PATH=../multiplier-dynamic/target/release:../adder go run . && cd -

