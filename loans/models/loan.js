const mongoose = require('mongoose');


const loanSchema = new mongoose.Schema(
    {
        id : {
            type:String,
            trim:true,
            required:true,
            max:64
        },
        amount:{
            type: Number,
            required:true
        },
        duration:{
            type: Number,
            required:true
        }
}, {timestamps: true});

module.exports = mongoose.model('Loan', loanSchema);