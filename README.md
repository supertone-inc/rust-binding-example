# Rust Binding Example

This project is aimed at testing and showing how to **bind a Rust project to the multiple languages**.

## Prerequisites

### Common

- [Rust](https://www.rust-lang.org/tools/install)
- [just](https://github.com/casey/just)

### For C++ Project

- [CMake](https://cmake.org/download) >= 3.16 (See [this](https://cgold.readthedocs.io/en/latest/first-step/installation.html) for installation.)

### For Node.js Project

- [Node.js](https://nodejs.org/en/download) >= 14 (Using [nvm](https://github.com/nvm-sh/nvm) is recommended.)

### For Python Project

- [Python](https://www.python.org/downloads) >= 3.8 (Using [pyenv](https://github.com/pyenv/pyenv#installation) is recommended.)
- [Poetry](https://python-poetry.org/docs/#installation)

## Getting Started

You can run following commands either in project's root directory or in each subproject's directory.

### Build

```shell
just build
```

### Test

```shell
just test
```

> The `test` command automatically runs `build` before being executed.
