run: build
	docker run -v `pwd`:/repo rustfix > output.txt 2>&1 || true

build:
	docker build -t rustfix .

