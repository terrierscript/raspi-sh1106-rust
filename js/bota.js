const dup = (base) => {
  return base.reduce((acc, ys, y) => {
    const newRow = ys.reduce((acc, v, x) => {
      return [...acc, v, v]
    }, [])
    return [...acc, newRow, newRow]
  }, [])
}
module.exports = (xOffset = 0) => {
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
  const character = dup(_character)
  const wSize = character[0].length
  const hSize = character.length
  const wPad = (w - wSize) / 2 + xOffset * 2
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
