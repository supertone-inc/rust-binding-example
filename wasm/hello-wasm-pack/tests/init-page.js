module.exports = async () => {
  await page.goto("http://localhost:3000");
  await page.evaluate(async () => {
    const { default: init, ...hello } = await import("/hello/index.js");
    await init();
    window.hello = hello;
  });
};
