const initPage = require("../init-page");

beforeAll(initPage);

test("throw_error", async () => {
  await expect(
    page.evaluate(() => {
      hello.throw_error();
    })
  ).rejects.toThrow("error from Rust!");
});
