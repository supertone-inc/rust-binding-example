const hello = require("..");

test("throwError", () => {
  expect(() => hello.error.throwError()).toThrow("error from Rust!");
});
