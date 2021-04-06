.PHONY: build
build:
	docker build -t clippy .

.PHONY: run
run: build
	docker run -v ${PWD}:/repo -ti clippy 2>&1 > output.txt