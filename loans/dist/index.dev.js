"use strict";

var express = require("express");

var app = express();

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
var url = "mongodb://".concat(MONGO_USERNAME, ":").concat(MONGO_PASSWORD, "@").concat(MONGO_HOSTNAME, ":").concat(MONGO_PORT, "/").concat(MONGO_DB, "?authSource=admin");
mongoose.connect(url, {
  useNewUrlParser: true
});
app.use(bodyParser.json());
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