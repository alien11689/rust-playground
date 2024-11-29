all:
	cd adder && make all && cd - 
	cargo clean
	LD_LIBRARY_PATH=`pwd`/adder cargo test
	LD_LIBRARY_PATH=`pwd`/adder cargo build --release
	rm -f demo
	LD_LIBRARY_PATH=adder gcc -o demo main.c -L target/release -lmultiplier
	LD_LIBRARY_PATH=target/release:adder ./demo
