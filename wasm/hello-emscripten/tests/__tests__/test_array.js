const initPage = require("../init-page");

beforeAll(initPage);

test("concat", async () => {
  const serdeResult = await page.evaluate(() => {
    const result = hello.concat([1, 2], [3, 4, 5]);

    if (!(result instanceof Float32Array)) {
      throw "incorrect result type";
    }

    return result;
  });

  expect(Object.values(serdeResult)).toEqual([1, 2, 3, 4, 5]);
});
