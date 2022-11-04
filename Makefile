build:
	wasm-pack build
	cd www && pnpm i && pnpm build

clean:
	cargo clean
	rm -rf ./pkg ./www/dist ./www/node_modules