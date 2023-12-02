CC = gcc

SRC=main.c
OBJ=main.o
SHARE_LIB = libmain.so

.PHONY: all
all: $(OBJ) $(SHARE_LIB)

$(OBJ): $(SRC)
	$(CC) -g -fPIC -c $(SRC) -o $(OBJ)

$(SHARE_LIB) : $(OBJ)
	$(CC) -shared $(OBJ) -o $(SHARE_LIB)