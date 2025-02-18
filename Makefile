CC=gcc
AR=ar
CFLAGS=-Wall -Wextra -fPIC
LDFLAGS=-lespeak-ng
STATIC_LIB_DIR=libs
STATIC_LIB=libphonememize.a
ESPEAK_CONFIG=--with-chinese --with-chinese-variant --with-japanese --with-japanese-hiragana --with-japanese-katakana --without-sonic --enable-static

all: espeak $(STATIC_LIB)

espeak: 
	mkdir -p $(STATIC_LIB_DIR)
	cd espeak-ng && ./autogen.sh && ./configure --prefix=$(PWD)/$(STATIC_LIB_DIR) $(ESPEAK_CONFIG) && make -j 10 && make install

phonememize.o: phonememize.c phonememize.h
	$(CC) $(CFLAGS) -c phonememize.c -o $(STATIC_LIB_DIR)/phonememize.o -L libs/

$(STATIC_LIB): phonememize.o
	mkdir -p $(STATIC_LIB_DIR)
	$(AR) rcs $(STATIC_LIB_DIR)/$(STATIC_LIB) $(STATIC_LIB_DIR)/phonememize.o
	if [ "$(UNAME)" = "Darwin" ]; then \
    lipo -info $(STATIC_LIB); \
    ar -t $(STATIC_LIB); \
  fi
	rm $(STATIC_LIB_DIR)/phonememize.o

clean:
	rm -f *.o $(STATIC_LIB_DIR)/$(STATIC_LIB)
