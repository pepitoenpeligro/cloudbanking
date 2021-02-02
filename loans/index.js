const express = require("express");
const app = express();
const mongoose = require('mongoose');
const bodyParser = require('body-parser');
const dataRoutes = require('./routes/loans');

require('dotenv').config({path: __dirname + '/.env'});

const {
    MONGO_USERNAME,
    MONGO_PASSWORD,
    MONGO_HOSTNAME,
    MONGO_PORT,
    MONGO_DB
} = process.env


const url = `mongodb://${MONGO_USERNAME}:${MONGO_PASSWORD}@${MONGO_HOSTNAME}:${MONGO_PORT}/${MONGO_DB}?authSource=admin&retryWrites=true&w=majority&ssl=false`;
mongoose.connect(url, {
    useNewUrlParser: true,
    useFindAndModify: true,
    useUnifiedTopology: true,
    useCreateIndex: true,
    useFindAndModify: false,
    autoCreate : true
})
.then((val) => {console.log("DB connected"); console.log(mongoose.connection.readyState);
})
.catch((err)=> console.error("DB Connection errror", err));


app.use(bodyParser.json());

app.use(function(req, res, next) {
    // res.header("Access-Control-Allow-Origin", "*");
    // res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
    next();
  });

app.use('/loans', dataRoutes);

app.use(function(request, response){
    response.status(400);
    response.json({
        error:{
            'name': 'Error',
            'message': 'Our api is working just here: /loans'
        }
    });
});

const PORT = process.env.PORT || 3034;
app.listen(PORT, () => {
    console.log(`Loans Microservice is running on port ${PORT}`);
});

module.exports = app;