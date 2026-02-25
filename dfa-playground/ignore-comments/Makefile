CXX      := g++
CXXFLAGS := -std=c++17

TARGET := strip.out
SRC    := main.cpp

$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRC)

clean:
	rm -f $(TARGET) *.o
