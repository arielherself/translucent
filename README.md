# <p align="center">[Translucent](https://the-boys.fandom.com/wiki/Translucent)</p>

**<p align="center">A general proxy service.</p>**

## Introduction

Translucent aims to be a proxy service with simple but effective obfuscation. It modifies packets using multiple strategies to make them recognizable to proxy servers while maximally maintaining the similarity to the original ones.

This repository is an implementation of Translucent client and server. As only a few of common use cases are covered, please open a PR if you would like to add more features to it. Your help is greatly appreciated.

> [!IMPORTANT]
> This work is a proof of concept, which means it may contain vulnerability in security, performance etc. You should not run any part of the code on a production server.

## Run

First install through Cargo:

```bash
cargo install translucent
```

Run local server:

```bash
tllocal
```

Run remote server:

```bash
tlserver
```

You can also build the binaries manually. Debug features and more detailed logs can be enabled by compilation flag `--features debug`.
