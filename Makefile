prepare:
	./configs/git/setup.sh
test-unit:
	cargo test -p broken-app
	cargo miri test -p broken-app
	valgrind --leak-check=full --show-leak-kinds=all cargo test -p broken-app
test-e2e:
	cargo run -p broken-app --bin demo
	cargo miri run -p broken-app --bin demo
	valgrind --leak-check=full --show-leak-kinds=all cargo run -p broken-app --bin demo
test-sanitizers:
	RUSTFLAGS="-Zsanitizer=address" cargo run -p broken-app --bin demo -Zbuild-std --target x86_64-unknown-linux-gnu
	RUSTFLAGS="-Zsanitizer=thread" cargo run -p broken-app --bin demo -Zbuild-std --target x86_64-unknown-linux-gnu
test-all:
	make test-unit
	make test-e2e
	make test-sanitizers