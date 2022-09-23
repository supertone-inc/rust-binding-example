build:
    just cpp/hello/build
    just nodejs/hello/build
    just python/hello/build
    just rust/hello/build
    just wasm/hello-emscripten/build
    just wasm/hello-wasm-pack/build

test:
    just cpp/hello/test
    just nodejs/hello/test
    just python/hello/test
    just rust/hello/test
    just wasm/hello-emscripten/test
    just wasm/hello-wasm-pack/test
