"use strict";

var _mongoose$connect;

function _defineProperty(obj, key, value) { if (key in obj) { Object.defineProperty(obj, key, { value: value, enumerable: true, configurable: true, writable: true }); } else { obj[key] = value; } return obj; }

var express = require("express");

var app = express();

var mongoose = require('mongoose');

var bodyParser = require('body-parser');

var dataRoutes = require('./routes/loans');

require('dotenv').config({
  path: __dirname + '/.env'
});

var _process$env = process.env,
    MONGO_USERNAME = _process$env.MONGO_USERNAME,
    MONGO_PASSWORD = _process$env.MONGO_PASSWORD,
    MONGO_HOSTNAME = _process$env.MONGO_HOSTNAME,
    MONGO_PORT = _process$env.MONGO_PORT,
    MONGO_DB = _process$env.MONGO_DB;
var url = "mongodb://".concat(MONGO_USERNAME, ":").concat(MONGO_PASSWORD, "@").concat(MONGO_HOSTNAME, ":").concat(MONGO_PORT, "/").concat(MONGO_DB, "?authSource=admin&retryWrites=true&w=majority&ssl=false");
mongoose.connect(url, (_mongoose$connect = {
  useNewUrlParser: true,
  useFindAndModify: true,
  useUnifiedTopology: true,
  useCreateIndex: true
}, _defineProperty(_mongoose$connect, "useFindAndModify", false), _defineProperty(_mongoose$connect, "autoCreate", true), _mongoose$connect)).then(function (val) {
  console.log("DB connected");
  console.log(mongoose.connection.readyState);
})["catch"](function (err) {
  return console.error("DB Connection errror", err);
});
app.use(bodyParser.json());
app.use(function (req, res, next) {
  // res.header("Access-Control-Allow-Origin", "*");
  // res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
  next();
});
app.use('/loans', dataRoutes);
app.use(function (request, response) {
  response.status(400);
  response.json({
    error: {
      'name': 'Error',
      'message': 'Our api is working just here: /loans'
    }
  });
});
var PORT = process.env.PORT || 3034;
app.listen(PORT, function () {
  console.log("Loans Microservice is running on port ".concat(PORT));
});
module.exports = app;