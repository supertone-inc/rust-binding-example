const initPage = require("../init-page");

beforeAll(initPage);

test("runs without crash", async () => {
  await page.evaluate(() => {
    hello.greet();
  });
});
