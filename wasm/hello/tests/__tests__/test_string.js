const initPage = require("../init-page");

beforeAll(initPage);

test("to_uppercase", async () => {
  await expect(
    page.evaluate(() => {
      return hello.to_uppercase("Hello World!");
    })
  ).resolves.toBe("HELLO WORLD!");
});
