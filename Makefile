CC = gcc
CFLAGS = -Wall -fPIC -c 

DYLD_LIBRARY_PATH = ./
LD_LIBRARY_PATH = ./

all: lib
	cargo build

lib:
	$(CC) hello.c -o libhello.a $(CFLAGS)

clean:
	rm -rf ./target libhello.a 
