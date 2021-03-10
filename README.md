# Rust Binding Example

This project is aimed at testing and showing how to **bind a Rust project to the multiple languages**.

## Prerequisites

### Common

- [Rust](https://www.rust-lang.org) (See [Getting started](https://www.rust-lang.org/learn/get-started) for installation.)
- [cargo-make](https://sagiegurari.github.io/cargo-make)

### For C++ Project

- [CMake](https://cmake.org/download) (See [this](https://cgold.readthedocs.io/en/latest/first-step/installation.html) for installation.)

### For Node.js Project

- [Node.js](https://nodejs.org/en/download) >= 10 (Using [nvm](https://github.com/nvm-sh/nvm) is recommended.)

### For Python Project

- [Python](https://www.python.org/downloads) >= 3.8 (Using [pyenv](https://github.com/pyenv/pyenv#installation) is recommended.)
- [Poetry](https://python-poetry.org/docs/#installation)

## Getting Started

You can run following commands either in project's root directory or in each subproject's directory.

### Build

```sh
cargo make build
```

### Test

```sh
cargo make test
```

> The `test` command automatically runs `build` before being executed.
