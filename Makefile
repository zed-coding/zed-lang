CARGO = cargo

build-sys:
	@cd ./build-sys
	@cargo install --path .
	@cd ..

compiler:
	@cd ./compiler
	@cargo install --path .
	@cd ..

all:
	$(MAKE) compiler
	$(MAKE) build-sys
	@echo "Zed is installed sucessfully!"