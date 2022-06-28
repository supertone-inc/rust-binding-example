const initPage = require("../init-page");

beforeAll(initPage);

test("map", async () => {
  await expect(
    page.evaluate(() => {
      return hello.map([1, 2, 3], (item) => item / 2);
    })
  ).resolves.toEqual([0.5, 1.0, 1.5]);
});
