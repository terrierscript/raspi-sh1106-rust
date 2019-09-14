console.log("start")
const { SH1106 } = require("sh1106")
const bota = require("./bota")

const pad = (input) => {
  return ("0" + input).slice(-2)
}

const getDate = () => {
  const date = new Date()

  return `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(
    date.getSeconds()
  )}`
}

const device = new SH1106()
console.log("xx")
let counter = 0
setInterval(() => {
  let contrast = counter
  console.log("interval", contrast)
  // Clear the canvas
  device.canvas.clear()

  // Draw the current time at [1, 1] with a size of 2
  // device.canvas.text(10, 10, contrast.toString(), 3)
  bota().map((ys, y) => {
    ys.map((v, x) => {
      device.canvas.set(x, y, v)
    })
  })

  // Update the display
  device.contrast(contrast)
  counter += 10
  device.refresh()
}, 1000)
