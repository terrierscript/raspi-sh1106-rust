const { SH1106 } = require("sh1106")

const rpio = require("rpio")
const {LOW} = rpio

const device = new SH1106()

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
  buttons.map(b => {
    const l = rpio.read(b)
    console.log(b, l)
    if (l === LOW) {
      console.log('Foo', b); // Called when button is pressed
    }
  })
}, 1000);

console.log("end")