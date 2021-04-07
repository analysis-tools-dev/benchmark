run: build
	docker run -v `pwd`:/repo tool-name > output.txt 2>&1 || true

build:
	docker build -t tool-name .

