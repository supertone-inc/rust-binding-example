const hello = require("..");

test("to_uppercase", () => {
  expect(hello.to_uppercase("Hello World!")).toBe("HELLO WORLD!");
});

test("concat", () => {
  expect(hello.concat([1, 2], [3, 4, 5])).toEqual([1, 2, 3, 4, 5]);
});

test("raise_error", () => {
  expect(() => hello.raise_error()).toThrow("error raised from Rust!");
});
