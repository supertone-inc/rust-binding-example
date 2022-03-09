build:
    just cpp/hello/build
    just nodejs/hello/build
    just python/hello/build
    just rust/hello/build

test:
    just cpp/hello/test
    just nodejs/hello/test
    just python/hello/test
    just rust/hello/test

setup-win32:
    rustup default stable-i686-pc-windows-msvc
    pyenv global 3.8.10-win32

setup-win64:
    rustup default stable-x86_64-pc-windows-msvc
    pyenv global 3.8.10
