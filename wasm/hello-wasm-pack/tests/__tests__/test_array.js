const initPage = require("../init-page");

beforeAll(initPage);

test("concat", async () => {
  await expect(
    page.evaluate(() => {
      const result = hello.concat([1, 2], [3, 4, 5]);

      if (!(result instanceof Float32Array)) {
        throw "incorrect result type";
      }

      return Array.from(result);
    })
  ).resolves.toEqual([1, 2, 3, 4, 5]);
});
