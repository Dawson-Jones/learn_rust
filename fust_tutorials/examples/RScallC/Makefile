TARGET=main
RAW = ${TARGET:=.rs}

CFILE = add.c

LIB = lib${CFILE:.c=.so}

RFLAGS := -ladd -L.

$(TARGET): $(RAW) $(LIB)
	rustc $< $(RFLAGS) -o $@

CFLAGS := -fPIC -shared
$(LIB): $(CFILE)
	gcc $(CFLAGS) $^ -o $@


.PHONY: clean

clean:
	rm $(TARGET) $(LIB)