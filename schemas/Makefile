clean:
	@echo "Cleaning generated files..."
	@rm -rf generated

generate-rs:
	@mkdir -p generated/rust
	@cd generated/rust && cargo init --lib --name schemas .
	@cd generated/rust && cargo add capnp
	@echo "Installing capnpc-rust..."
	@cargo install capnpc --version 0.18.0
	@echo "Generating rust schemas..."
	@capnp compile -o rust:generated/rust/src person.capnp
	@echo "Globbing rust schemas and adding them to lib.rs..."
	@rm generated/rust/src/lib.rs
	@cd generated/rust && ls src/*.rs | sed 's/src\///g' | sed 's/\.rs//g' | xargs -I {} echo "pub mod {};" >> src/lib.rs
	@echo "Generating rust schemas done!"

generate: clean generate-rs