all: access_manager

access_manager: 
	cd access-manager && $(MAKE)
	cp ./target/wasm32-unknown-unknown/release/access_manager.wasm ./access_manager.wasm

access_manager.wasm.gz:
	cat ./access_manager.wasm | gzip -9 > ./access_manager.wasm.gz

.PHONY: access_manager_compile_optimized _access_manager_compile_optimized
access_manager_compile_optimized: _access_manager_compile_optimized access_manager.wasm.gz
_access_manager_compile_optimized:
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
	@# The following line is not necessary, may work only on linux (extra size optimization)
	wasm-opt -Oz ./target/wasm32-unknown-unknown/release/access_manager.wasm -o ./access_manager.wasm	