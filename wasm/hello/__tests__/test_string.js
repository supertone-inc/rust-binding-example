const initPage = require("../init-page");

beforeAll(initPage);

test("to_uppercase", async () => {
  const result = await page.evaluate(() => hello.to_uppercase("Hello World!"));
  expect(result).toBe("HELLO WORLD!");
});
