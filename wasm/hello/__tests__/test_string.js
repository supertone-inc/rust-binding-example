const hello = require("hello-wasm");

test("to_uppercase", () => {
  expect(hello.to_uppercase("Hello World!")).toBe("HELLO WORLD!");
});
