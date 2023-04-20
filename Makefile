build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen target/wasm32-unknown-unknown/release/css_typing_gen.wasm --out-dir=pkg
	cd www && yarn install && yarn build

start: build
	cd www && yarn preview

dev: build
	cd www && yarn start

clean:
	cargo clean
	rm -rf ./pkg ./www/dist ./www/node_modules