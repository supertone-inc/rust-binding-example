const initPage = require("../init-page");

beforeAll(initPage);

test("concat", async () => {
  await expect(
    page.evaluate(() => {
      return hello.concat([1, 2], [3, 4, 5]);
    })
  ).resolves.toEqual([1, 2, 3, 4, 5]);
});
