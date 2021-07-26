const hello = require("..");

test("to_uppercase", () => {
  expect(hello.string.to_uppercase("Hello World!")).toBe("HELLO WORLD!");
});
