const hello = require("../hello");

test("map", () => {
  expect(hello.callback.map([1, 2, 3], (item) => item / 2)).toEqual([
    0.5, 1.0, 1.5,
  ]);
});
