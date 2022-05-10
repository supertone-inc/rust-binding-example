const hello = require("..");

test("raiseError", () => {
  expect(() => hello.error.raiseError()).toThrow("error raised from Rust!");
});
