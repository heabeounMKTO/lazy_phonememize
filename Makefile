CC=gcc
AR=ar
CFLAGS=-Wall -Wextra -fPIC
LDFLAGS=-lespeak-ng
STATIC_LIB_DIR=libs
STATIC_LIB=libphonememize.a
OBJ_FILE=phonememize.o

all: espeak $(STATIC_LIB)

espeak: 
	mkdir -p $(STATIC_LIB_DIR)
	cd espeak-ng && ./autogen.sh && ./configure --prefix=$(PWD)/$(STATIC_LIB_DIR) && make -j 10 && make install

$(STATIC_OBJ): phonememize.c phonememize.h
	$(CC) $(CFLAGS) -c phonememize.c -o $(STATIC_LIB_DIR)/$(STATIC_OBJ) -L libs/

$(STATIC_LIB): $(STATIC_OBJ)
	mkdir -p $(STATIC_LIB_DIR)
	$(AR) rcs $(STATIC_LIB_DIR)/$(STATIC_LIB) $(STATIC_LIB_DIR)/$(STATIC_OBJ)
	if [ "$(UNAME)" = "Darwin" ]; then \
    lipo -info $(STATIC_LIB); \
    ar -t $(STATIC_LIB); \
  fi
	rm $(STATIC_LIB_DIR)/$(STATIC_OBJ)

clean:
	rm -f *.o $(STATIC_LIB_DIR)/$(STATIC_LIB)
