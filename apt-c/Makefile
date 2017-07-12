CXXFLAGS=-Wall -Wextra

all: libapt-c.a

lib.o: lib.cpp lib.h

libapt-c.a: lib.o
	ar rcs $@ $<

clean:
	$(RM) libapt-c.a lib.o
