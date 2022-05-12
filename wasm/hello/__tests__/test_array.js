const initPage = require("../init-page");

beforeAll(initPage);

test("concat", async () => {
  const result = await page.evaluate(() => hello.concat([1, 2], [3, 4, 5]));
  expect(result).toEqual([1, 2, 3, 4, 5]);
});
