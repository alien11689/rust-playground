all: static

static:
	cd adder && make all && cd - 
	cd multiplier-static && cargo clean && cargo test && cargo build --release
	rm -f demo-static
	gcc -o demo-static main.c -L multiplier-static/target/release -lmultiplier
	./demo-static
