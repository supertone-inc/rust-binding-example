const hello = require("..");

test("toUppercase", () => {
  expect(hello.string.toUppercase("Hello World!")).toBe("HELLO WORLD!");
});
