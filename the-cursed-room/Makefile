SOURCE=main.cpp
OUTPUT=TheCursedRoom
INCLUDES=
LIBRARIES=
CC=clang++
CCFLAGS=-stdlib=libc++ -std=c++11

#------------------------------------------------------------------------------

all: $(OUTPUT)
	
$(OUTPUT): $(SOURCE)
	$(CC) $(CCFLAGS) $(INCLUDES) $(SOURCE) -o$(OUTPUT) $(LIBRARIES)

clean:
	rm -f $(OUTPUT)