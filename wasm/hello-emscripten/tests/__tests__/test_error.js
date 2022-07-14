const initPage = require("../init-page");

beforeAll(initPage);

test("throwError", async () => {
  await expect(
    page.evaluate(() => {
      try {
        hello.throwError();
      } catch (error) {
        throw typeof error === "number"
          ? new Error(hello.getExceptionMessage(error))
          : error;
      }
    })
  ).rejects.toThrow("error from Rust!");
});
