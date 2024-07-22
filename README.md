# <p align="center">[Translucent](https://the-boys.fandom.com/wiki/Translucent)</p>

<p align="center">A general proxy service.</p>

## Goal

Translucent aims to be a proxy service with simple but effective obfuscation. It modifies packets using multiple strategies to make them recognizable to proxy servers while maximally maintaining the similarity to the original ones.

## Run

Run local server:

```bash
RUST_LOG=info cargo run --release --bin tllocal
```

Run remote server:

```bash
RUST_LOG=info cargo run --release --bin tlserver
```
