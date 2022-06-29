const initPage = require("../init-page");

beforeAll(initPage);

test("toUppercase", async () => {
  await expect(
    page.evaluate(() => {
      return hello.toUppercase("Hello World!");
    })
  ).resolves.toBe("HELLO WORLD!");
});
