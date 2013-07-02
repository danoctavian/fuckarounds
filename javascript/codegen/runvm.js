var vm = require("vm"),
    util = require("util")
;
console.log("making CODE")
var script = vm.createScript("console.log(12345)")
console.log(util.inspect(script))
script.runInThisContext()

var json = JSON.stringify(script)
//console.log(util.inspect(json))

function f() {
  console.log("wtf")
}

console.log(script + "")


