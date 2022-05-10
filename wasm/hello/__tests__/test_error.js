const hello = require("hello-wasm");

test("throw_error", () => {
  expect(() => hello.throw_error()).toThrow("error from Rust!");
});
