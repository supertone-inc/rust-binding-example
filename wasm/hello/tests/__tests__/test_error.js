const initPage = require("../init-page");

beforeAll(initPage);

test("throw_error", async () => {
  const result = await page.evaluate(() => {
    try {
      hello.throw_error();
    } catch (error) {
      return error.message;
    }
  });
  expect(result).toBe("error from Rust!");
});
