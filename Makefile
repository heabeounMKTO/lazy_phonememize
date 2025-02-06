CC=gcc
AR=ar
CFLAGS=-Wall -Wextra -fPIC
LDFLAGS=-lespeak-ng
STATIC_LIB_DIR=libs


all: espeak libphonememize.a

espeak: 
	mkdir -p $(STATIC_LIB_DIR)
	cd espeak-ng && ./autogen.sh && ./configure --prefix=$(PWD)/$(STATIC_LIB_DIR) && make -j 10 && make install

phonememize.o: phonememize.c phonememize.h
	$(CC) $(CFLAGS) -c phonememize.c -o $(STATIC_LIB_DIR)/phonememize.o -L libs/

libphonememize.a: phonememize.o
	mkdir -p $(STATIC_LIB_DIR)
	$(AR) rcs $(STATIC_LIB_DIR)/libphonememize.a $(STATIC_LIB_DIR)/phonememize.o
	rm $(STATIC_LIB_DIR)/phonememize.o

clean:
	rm -f *.o $(STATIC_LIB_DIR)/libphonememize.a
