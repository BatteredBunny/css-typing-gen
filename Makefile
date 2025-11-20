build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen target/wasm32-unknown-unknown/release/css_typing_gen.wasm --out-dir=pkg
	cd www && pnpm install && pnpm build

start: build
	cd www && pnpm preview

dev: build
	cd www && pnpm start

clean:
	cargo clean
	rm -rf ./pkg ./www/dist ./www/node_modules