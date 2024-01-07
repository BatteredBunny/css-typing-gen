
<h1 align="center">Website for generating CSS typing animations</h1>

https://github.com/BatteredBunny/css-typing-gen/assets/52951851/0b76b304-264a-4caa-a83a-f54b87a1bd6c

# Dev instructions

## Required dependencies
Rust, wasm-bindgen, nodejs, yarn, gnumake

## Preview the website
```
make start
```

## Building the website
```
make build
```

Built files will be in www/dist

# Nix
## Preview the website
```
nix develop
make start
```

## Building the website
```
nix build
```
Built files will be in result/
