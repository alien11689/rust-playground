all:
	cd adder && make all && cd - 
	cargo clean
	cargo test
	cargo build --release
	rm -f demo
	gcc -o demo main.c -L target/release -lmultiplier
	./demo
