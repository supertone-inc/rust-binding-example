const initPage = require("../init-page");

beforeAll(initPage);

test("concat", async () => {
  await expect(
    page.evaluate(() => {
      return Array.from(hello.concat([1, 2], [3, 4, 5]));
    })
  ).resolves.toEqual([1, 2, 3, 4, 5]);
});

test("concatPreallocated", async () => {
  await expect(
    page.evaluate(() => {
      const a = hello.Float32Vector.from([1, 2]);
      const b = hello.Float32Vector.from([3, 4, 5]);
      const c = new hello.Float32Vector(5);

      hello.concatPreallocated(a, b, c);

      return Array.from(c.toTypedArray());
    })
  ).resolves.toEqual([1, 2, 3, 4, 5]);
});

test("Float32Vector", async () => {
  await expect(
    page.evaluate(() => {
      const vector = new hello.Float32Vector(3);

      vector.toTypedArray().set([1, 2, 3]);

      return Array.from(vector.toTypedArray());
    })
  ).resolves.toEqual([1, 2, 3]);
});
