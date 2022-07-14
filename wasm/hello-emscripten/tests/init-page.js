module.exports = async () => {
  await page.goto("http://localhost:3000");
  await page.evaluate(async () => {
    const { default: init } = await import("/hello/index.js");
    const hello = await init();
    window.hello = hello;
  });
};
