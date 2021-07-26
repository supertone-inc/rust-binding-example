const hello = require("../hello");

test("raise_error", () => {
  expect(() => hello.error.raise_error()).toThrow("error raised from Rust!");
});
