const initPage = require("../init-page");

beforeAll(initPage);

test("throwError", async () => {
  await expect(
    page.evaluate(() => {
      hello.throwError();
    })
  ).rejects.toThrow("error from Rust!");
});
