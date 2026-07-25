GCC = gcc
AR = ar
RUSTC = rustc

CFLAGS = -O3 -std=c11 -Wall -Wextra -Werror
RUSTFLAGS = -O

TARGET = main
LIB = libeuler23.a
OBJ = main.o

all: $(TARGET)

$(OBJ): main.c
	$(GCC) $(CFLAGS) -c main.c -o $(OBJ)

$(LIB): $(OBJ)
	$(AR) rcs $(LIB) $(OBJ)

$(TARGET): main.rs $(LIB)
	$(RUSTC) $(RUSTFLAGS) main.rs -L . -l static=euler23 -o $(TARGET)

clean:
	rm -f $(OBJ) $(LIB) $(TARGET)