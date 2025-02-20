CARGO = cargo

build-sys:
	@cd ./build-sys
	@cargo install --path .
	@cd ..

compiler:
	@cd ./compiler
	@cargo install --path .
	@cd ..

docgen:
	@cd ./tools/zed-docgen
	@cargo install --path .
	@cd -

fmt:
	@cd ./tools/zed-fmt
	@cargo install --path .
	@cd -

pkg:
	@cd ./tools/zed-pkg
	@cargo install --path .
	@cd -

all:
	$(MAKE) compiler
	$(MAKE) build-sys
	$(MAKE) docgen
	$(MAKE) fmt
	$(MAKE) pkg
	@echo "Zed is installed sucessfully!"
