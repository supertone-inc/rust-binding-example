const { sleep, sync } = require('..')

test('sync function from native code', () => {
  const fixture = 42
  expect(sync(fixture)).toBe(fixture + 100)
})

test('sleep function from native code', async () => {
  const timeToSleep = 200
  const value = await sleep(timeToSleep)
  expect(value).toBe(timeToSleep * 2)
})
