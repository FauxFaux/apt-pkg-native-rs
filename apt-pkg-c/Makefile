CXXFLAGS=-Wall -Wextra -g

all: libapt-pkg-c.a

lib.o: lib.cpp lib.h

libapt-pkg-c.a: lib.o
	ar rcs $@ $<

clean:
	$(RM) libapt-pkg-c.a lib.o
