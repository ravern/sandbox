all: build
build: src/*.java Main.java
	javac -d dist/ src/*.java Main.java
run: build
	java -cp dist/ Main
clean:
	rm -rf dist/*.class