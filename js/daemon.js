const { SH1106 } = require("sh1106")
const os = require("os")

const device = new SH1106()

setInterval(() => {
  // Clear the canvas
  device.canvas.clear()

  // Draw the current time at [1, 1] with a size of 2
  device.canvas.text(1, 1, os.arch(), 1)
  const cpus = os.cpus()
  if (cpus[0].times) {
    device.canvas.text(10, 1, cpus[0].times.user, 1)
  }

  // Update the display
  device.refresh()
}, 1000)
