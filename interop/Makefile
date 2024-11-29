all: static dynamic

adder:
	cd adder && make all && cd - 

static: adder
	cd multiplier-static && cargo clean && cargo test && cargo build --release
	rm -f demo-static
	gcc -o demo-static main.c -L multiplier-static/target/release -lmultiplier
	./demo-static

dynamic: adder
	LD_LIBRARY_PATH=`pwd`/adder cd multiplier-dynamic && cargo clean && cargo test && cargo build --release
	rm -f demo-dynamic
	LD_LIBRARY_PATH=adder gcc -o demo-dynamic main.c -lmultiplier -L multiplier-dynamic/target/release
	LD_LIBRARY_PATH=multiplier-dynamic/target/release:adder ./demo-dynamic