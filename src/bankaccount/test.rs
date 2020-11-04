use crate::bankaccount::model::{Account};
use chrono::{NaiveDate, NaiveDateTime};

#[cfg(test)]
/// Test for Bankaccount
mod test_bankaccount{
    use super::*;

    #[test]
    /// This test check if a BankAccount can set name
    fn test_name(){
        let to_expect = "Jose";
        let bankAccount = "{\"name\" : \"Jose Antonio\"}";
        assert!(bankAccount.contains(to_expect));
    }

    #[test]
    /// Test if ID assignment is correct
    fn test_get_id(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES326688051111222233344");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_eq!(*acc.get_id(),String::from("507f1f77bcf86cd799439011"));
    }


    #[test]
    /// Test if swift assignment is correct
    fn test_get_swift(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES326688051111222233344");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_eq!(*acc.get_swift(),String::from("BSCHESMMXXX"));
    }
}