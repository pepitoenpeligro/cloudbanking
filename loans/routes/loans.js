const express = require('express');
const router = express.Router();

const {getLoan, postLoan} = require('../controllers/loansController.js');

router.get('/', getLoan);
router.post('/', postLoan);

module.exports = router;