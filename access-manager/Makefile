all: src/contract.rs src/lib.rs src/msg.rs src/state.rs
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

clean:
	cargo clean
	-rm -f ./contract.wasm