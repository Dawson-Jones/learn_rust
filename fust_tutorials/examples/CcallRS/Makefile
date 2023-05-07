CC = clang

TARGET=main
RAW = ${TARGET:=.c}

RSFILE = add.rs

LIB = lib${RSFILE:.rs=.so}

CFLAGS := -ladd -L.

$(TARGET): $(RAW) $(LIB)
	$(CC) $< $(CFLAGS) -o $@

RSFLAGS := --crate-type=cdylib
$(LIB): $(RSFILE)
	rustc $(RSFLAGS) $^ -o $@


.PHONY: clean
clean:
	rm $(TARGET) $(LIB)