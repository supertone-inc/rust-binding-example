const hello = require("hello-wasm");

test("concat", () => {
  expect(hello.concat([1, 2], [3, 4, 5])).toEqual([1, 2, 3, 4, 5]);
});
