.PHONY: build
build-mac:
	cargo build --release
	rm -f ./lua/timer.so
	cp ./target/release/libtimer.dylib ./lua/timer.so
	# if your Rust project has dependencies,
	# you'll need to do this as well
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
build-arch:
	cargo build --release
	rm -f ./lua/timer.so
	cp ./target/release/libtimer.so ./lua/timer.so
	# if your Rust project has dependencies,
	# you'll need to do this as well
	mkdir -p ./lua/deps/
	cp ./target/release/deps/*.rlib ./lua/deps/
