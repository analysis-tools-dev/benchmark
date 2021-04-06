.PHONY: build
build:
	docker build -t clippy .

.PHONY: run
run: build
	docker run -v ${CURDIR}:/repo clippy > output.txt 2>&1 || true