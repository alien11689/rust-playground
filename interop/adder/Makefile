all: clean static dynamic

adder.o:
	gcc -c -Wall -Werror adder.c -o adder.o

static: adder.o
	ar rcs libadder.a adder.o

dynamic: adder.o
	gcc -shared -o libadder.so adder.o

clean:
	rm -f *.o *.so *.a *.gch

