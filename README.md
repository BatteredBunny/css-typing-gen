<h1 align="center">css-typing-gen</h1>
<p align="center">Generate CSS typing animations</p>

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