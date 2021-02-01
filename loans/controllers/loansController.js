const Loan = require('../models/loan');


exports.getLoan = (request, response) => {
    console.log("Getting loans");

    Loan.find({}, function(err, loansrecovered) {
        var loansLocal = [];
    
        loansrecovered.forEach(function(loan) {
          loansLocal.push(loan)
        });
        response.status(200);
        response.header("Content-Type",'application/json');
        response.send(loansLocal);  
      });
}

exports.postLoan = (request, response) => {
    console.log("Posting loans");
    const id = request.body.id;
    const amount = request.body.amount;
    const duration = request.body.duration;

    let newLoan = new Loan({id:id, amount:amount, duration:duration});

    response.header("Content-Type",'application/json');
    newLoan.save(function (err, s){
        if(err){
            console.log(err);
            return response.status(404).json(
                err
            )
        }
        return response.status(201).json({
            msg: "Loand was created successfully"
        });
    });

}
