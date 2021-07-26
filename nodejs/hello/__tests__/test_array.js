const hello = require("../hello");

test("concat", () => {
  expect(hello.array.concat([1, 2], [3, 4, 5])).toEqual([1, 2, 3, 4, 5]);
});
