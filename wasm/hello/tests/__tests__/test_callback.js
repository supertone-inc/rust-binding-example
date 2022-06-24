const initPage = require("../init-page");

beforeAll(initPage);

test("map", async () => {
  const result = await page.evaluate(() =>
    hello.map([1, 2, 3], (item) => item / 2)
  );
  expect(result).toEqual([0.5, 1.0, 1.5]);
});
