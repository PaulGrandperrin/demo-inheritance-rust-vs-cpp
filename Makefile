build: main

run: main
	./main

main: src/main.cpp
	clang++ -std=c++11 src/main.cpp -o main

clean:
	rm -f ./main
