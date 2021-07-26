const hello = require("..");

test("hello", () => {
  expect(hello.hello()).toBe("hello node");
});
