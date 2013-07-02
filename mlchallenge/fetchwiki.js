var http = require('http')
var req = require('request')
var util = require('util')
var htmlparser = require("htmlparser")
var dco = require("./dcoutils.js")

console.log(Array.prototype.append)

var rawHtml = "<html> <h1> HEAD </h1> <p>wtf man </p> <p> fuark </p> </html> "

var handler = new htmlparser.DefaultHandler(function (error, dom) {
    if (error) {
      console.log("error")
    }
    else {
      console.log("parsing complete")
    }
});
var parser = new htmlparser.Parser(handler)
parser.parseComplete(rawHtml)

/* returns an array of raw text strings */

function applyToNode(node, f) {
  f(node)
  var ch = node.children
  if (ch) {
    for (var i = 0; i < ch.length; i++) {
      applyToNode(ch[i], f)
    }
  }
} 

function getRawText(root) {
  raw = []
  applyToNode(root, function (node) {
    if (node.type === "text") {raw.push(node.raw)}
  })
  return raw
}


console.log(getRawText(handler.dom[0]))
//console.log(util.inspect(handler.dom, false, null))

process.exit(0)

var host = "http://en.wikipedia.org"
var api = "/w/api.php?"
//var mpage = "/w/api.php?format=json&action=query&titles=Main%20Page&prop=revisions&rvprop=content"

/* pref -string, args - obj */
function getReqArgs(args) { 
  var reqUrl = ""
  for (k in args) { reqUrl += k + "=" + args[k] + "&"}
  return reqUrl.substr(0, reqUrl.length - 1)
}

function getUrl(args) { return host + api + getReqArgs(args) }

var pageName = "Tree"
var mpage = {format: "json", action: "query", titles: "Main%20Page",
            prop: "revisions", rvprop: "content"}
var r = getUrl(mpage)
console.log(r)

req(r, function(error, response, body) {
  //console.log("@@@@@@@@@@le body " + body)
  var resp = JSON.parse(body)
  var page = null
  for (var k in resp.query.pages) {
    page = resp.query.pages[k]
  }
  console.log(page.title) 
  for (var k in page) {
    console.log(k) 
  }

  var html = page.revisions[0]["*"]
  var html = page.revisions[0]["*"]
  console.log(html)
})

/*
http.get(options, function(res) {
  res.on("data", function(chunk) {
    console.log("got data " + chunk)
  })
  console.log("Got response: " + res.statusCode)
  console.log(res.headers)
}).on('error', function(e) {
  console.log("Got error: " + e.message)
});
*/
console.log("reading from wikipedia")
