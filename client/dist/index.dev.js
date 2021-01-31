"use strict";

var express = require("express");

var http = require("http");

var path = require("path");

var app = express();
app.use(express["static"](path.join(__dirname, "build")));
app.get("/", function (req, res) {
  res.sendFile(path.join(__dirname, "build", "index.html"));
});
var server = http.createServer(app);
var PORT = process.env.port || 8080;
server.listen(PORT);
server.on("listening", function () {
  console.log("Servidor escuchando en el puerto: ".concat(PORT));
});