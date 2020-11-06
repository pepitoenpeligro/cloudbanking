
mod test_bankaccount_galvanic{
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;

    use chrono::{NaiveDate, NaiveDateTime};
    use crate::bankaccount::model::bankaccount::*;
    use crate::bankcard::model::{Card};

    #[test]
    /// Test if ID assignment is correct (inmutable getter - constructor)
    fn test_get_id(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_that!(acc.get_id(), eq(String::from("507f1f77bcf86cd799439011")));
    }


    #[test]
    /// Test if constructor is correct
    fn test_new_account(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_that!(&acc, is_variant!(Account));
    }


}