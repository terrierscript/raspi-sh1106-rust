const dup = (base, size) => {
  return base.reduce((acc, ys, y) => {
    const newRow = ys.reduce((acc, v, x) => {
      return [...acc, ...Array(size).fill(v)]
    }, [])
    return [...acc, ...Array(size).fill(newRow)]
  }, [])
}
module.exports = (xOffset = 0, scale = 2) => {
  const w = 128
  const h = 64
  const _character = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 1, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 0, 0, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1]
  ]
  const character = dup(_character, scale)
  const wSize = character[0].length
  const hSize = character.length
  const wPad = (w - wSize) / 2 + xOffset * scale * 2
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
