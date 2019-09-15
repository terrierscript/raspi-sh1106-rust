const { SH1106 } = require("sh1106")
const os = require("os")

const device = new SH1106()

setInterval(() => {
  // Clear the canvas
  device.canvas.clear()
  const lines = [
    `arch: ${os.arch()}`,
    `cpu: ${os.cpus()[0].times.user}`,
    `mem: ${os.freemem()}`
  ]

  // Draw the current time at [1, 1] with a size of 2
  lines.map((l, i) => {
    device.canvas.text(1, i * 10, l.toString(), 1)
  })

  // Update the display
  device.refresh()
}, 1000)
