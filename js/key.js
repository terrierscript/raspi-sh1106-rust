
const rpio = require("rpio")
const {LOW} = rpio

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

// buttons.map(b => {
  
  // rpio.poll(21, cb)
// })
setInterval(() => {
  if (rpio.read(21) === LOW) {
    console.log('Foo'); // Called when button is pressed
  }
}, 10);

console.log("end")