
const rpio = require("rpio")
const {LOW} = rpio

console.log("start")
const buttons = [
  21,
  // 20,
  16,
  // 6,
  19,
  5,
  26,
]
buttons.map(b => {
  console.log(b)
  rpio.open(b, rpio.INPUT,rpio.PULL_UP);
})
// rpio.open(20, rpio.INPUT, rpio.PULL_UP);

const cb = (cbpin) => {
  console.log("button",cbpin)
}

// buttons.map(b => {
  
  // rpio.poll(21, cb)
// })
setInterval(() => {
  console.log(rpio.read(21))
  buttons.map(b => {
    const l = rpio.read(b)
    console.log(b, l)
    if (l === LOW) {
      console.log('Foo', b); // Called when button is pressed
    }
  })
}, 100);

console.log("end")