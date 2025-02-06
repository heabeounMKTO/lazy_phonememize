CC=gcc
AR=ar
CFLAGS=-Wall -Wextra -fPIC
LDFLAGS=-lespeak-ng
STATIC_LIB_DIR=libs


all: espeak libphonememize.a

espeak: 
	cd espeak-ng && ./autogen.sh && ./configure --prefix=$(PWD)/libs && make -j 10 && make install

phonememize.o: phonememize.c phonememize.h
	$(CC) $(CFLAGS) -c phonememize.c

libphonememize.a: phonememize.o
	mkdir -p $(STATIC_LIB_DIR)
	$(AR) rcs $(STATIC_LIB_DIR)/libphonememize.a phonememize.o
	rm phonememize.o

clean:
	rm -f *.o libphonememize.a
