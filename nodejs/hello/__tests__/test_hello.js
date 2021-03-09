const hello = require('..')

test('to_uppercase', () => {
  expect(hello.to_uppercase('Hello World!')).toBe('HELLO WORLD!')
})
