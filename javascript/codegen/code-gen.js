/* codegen speed vs normal speed */
function doub(x) { return x * 2}

var genedDoub = new Function("x", "return x * 2")
var time = new Date()

function runFunc(f, elems) {
  var start = new Date() 
  for (var i = 0; i < elems.length; i++) {
    var res = f(elems[i])
  }
  var end = new Date()
  return end.getTime() - start.getTime()
}

var a = []
for (var i = 0; i < 10000000; i++) {
  a.push(Math.random())
}

console.log(runFunc(doub, a))
console.log(runFunc(genedDoub, a))


