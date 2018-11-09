// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

test('description of example test', (t) => {
  // indicates the number of assertions that follow
  t.plan(1)

  // Make a call to a Zome function
  // indicating the capability and function, and passing it an input
  const result = app.call("hosts", "main", "log_service", `"input"`)

  // check for equality of the actual and expected results
  t.equal(result, "expected result!")
})
