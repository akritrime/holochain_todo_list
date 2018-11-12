// This test file uses the tape testing framework. 
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

// test('description of example test', (t) => {
//   // indicates the number of assertions that follow
//   t.plan(1)

//   // Make a call to a Zome function
//   // indicating the capability and function, and passing it an input
//   const result = app.call("zome-name", "capability-name", "function-name", "input-data")

//   // check for equality of the actual and expected results
//   t.equal(result, "expected result!")
// })


test('Can create a list', (t) => {
  const create_result = app.call("lists", "main", "create_list", JSON.stringify({name: "test list"}))
  // t.equal(JSON.parse(create_result).success, true)
  console.log(create_result)
  t.end()
})

test('Can add some items', (t) => {
  const create_result = app.call("lists", "main", "create_list", JSON.stringify({list: {name: "test list"}}))
  const list_addr = JSON.parse(create_result).address
  console.log(list_addr)

  const result1 = app.call("lists", "main", "add_item", JSON.stringify({list_item: {text: "Learn Rust", completed: true}, list_addr: list_addr}))
  const result2 = app.call("lists", "main", "add_item", JSON.stringify({list_item: {text: "Master Holochain", completed: false}, list_addr: list_addr}))

  console.log(result1)
  console.log(result2)

  // t.equal(JSON.parse(result1).success, true)
  // t.equal(JSON.parse(result2).success, true)

  t.end()
})

test('Can get a list with items', (t) => {
  const create_result = app.call("lists", "main", "create_list", JSON.stringify({list: {name: "test list"}}))
  const list_addr = JSON.parse(create_result).address

  app.call("lists", "main", "add_item", JSON.stringify({list_item: {text: "Learn Rust", completed: true}, list_addr: list_addr}))
  app.call("lists", "main", "add_item", JSON.stringify({list_item: {text: "Master Holochain", completed: false}, list_addr: list_addr}))

  const get_result = app.call("lists", "main", "get_list", JSON.stringify({list_addr: list_addr}))

  // t.equal(JSON.parse(get_result).result.length, 2)
  console.log(get_result)
  t.end()
})