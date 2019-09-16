
const rpio = require("rpio")

console.log("start")
const buttons = [
  21,
  // 20,
  // 16,
  // 6,
  // 19,
  // 5,
  // 26,
]
// buttons.map(b => {
  //   console.log(b)
  // })
  rpio.open(21, rpio.INPUT,rpio.PULL_UP);
  // rpio.open(20, rpio.INPUT, rpio.PULL_UP);
  
  const cb = (cbpin) => {
    console.log("button",cbpin)
  }
  
  buttons.map(b => {
    rpio.poll(b, cb)
  })
  
  console.log("end")