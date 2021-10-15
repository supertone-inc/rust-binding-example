build:
    just cpp/hello/build
    just nodejs/hello-napi/build
    just nodejs/hello-neon/build
    just python/hello/build
    just rust/hello/build

test:
    just cpp/hello/test
    just nodejs/hello-napi/test
    just nodejs/hello-neon/test
    just python/hello/test
    just rust/hello/test
