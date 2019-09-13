console.log("start")
const { SH1106 } = require("sh1106")

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
setInterval(() => {
  console.log("interval")
  // Clear the canvas
  device.canvas.clear()

  // Draw the current time at [1, 1] with a size of 2
  device.canvas.text(1, 1, "aaa", 1)

  // Update the display
  device.refresh()
}, 1000)
