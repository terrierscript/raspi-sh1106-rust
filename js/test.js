const bota = require("./bota")

const r = bota()
const z = r.map((rr) => rr.join("")).join("\n")
console.log(z)
