console.log("loading dcoutils..")

Array.prototype.append =
function(a) {
  this.push.apply(this, a)
}


