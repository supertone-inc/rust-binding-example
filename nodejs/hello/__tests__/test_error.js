const hello = require("..");

test("raise_error", () => {
  expect(() => hello.error.raise_error()).toThrow("error raised from Rust!");
});
