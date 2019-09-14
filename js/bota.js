module.exports = () => {
  const w = 128
  const h = 64
  const character = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 1, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 0, 0, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1]
  ]
  const wSize = character[0].length
  const hSize = character.length
  const wPad = (w - wSize) / 2
  const hPad = h - hSize
  let map = Array(h)
    .fill(0)
    .map(() => Array(w).fill(0))
  character.map((ys, y) => {
    ys.map((val, x) => {
      map[y + hPad][x + wPad] = val
    })
  })
  return map
}
