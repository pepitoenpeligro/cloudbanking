
mod test_bankaccount_galvanic{
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;

    use chrono::{NaiveDate, NaiveDateTime};
    use crate::bankaccount::model::bankaccount::*;
    use crate::bankcard::model::{Card};

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
    /// Test if swift assignment is correct (inmutable getter - constructor)
    fn test_get_swift(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_that!(&acc.get_swift().clone(), is(eq(String::from("BSCHESMMXXX"))));
    }

    #[test]
    /// Test if iban assignment is correct (inmutable getter - constructor)
    fn test_get_iban(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_that!(&acc.get_iban().clone(), is(eq(String::from("ES32668805111122223334"))));
    }

    #[test]
    /// Test if created_at - date assignment is correct (inmutable getter - constructor)
    fn test_get_date(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_that!(&acc.get_date().clone(), is(eq(NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0))));
    }

    #[test]
    /// Test if status assignment is correct (inmutable getter - constructor)
    fn test_get_status(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_that!(&acc.get_status().clone(), is(eq(true)));
    }

    #[test]
    /// Test if swift assignment is correct (inmutable getter - setter)
    fn test_set_swift(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let mut acc     : Account           = Account::new(id, swift, iban, date, status);

        let new_swift   : String            = String::from("BBVAESMMXXX");
        acc.set_swift(new_swift.clone());
        assert_that!(&acc.get_swift(), is(eq(String::from("BBVAESMMXXX"))));
        assert_that!(&acc.get_swift(), not(is(eq(String::from("BSCHESMMXXX")))));
    }

    #[test]
    /// Test if iban assignment is correct (inmutable getter - setter)
    fn test_set_iban(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let mut acc     : Account           = Account::new(id, swift, iban, date, status);

        let new_iban  : String            = String::from("ES49998877666655554440");
        acc.set_iban(new_iban);
        assert_that!(&acc.get_iban().clone(), is(eq(String::from("ES49998877666655554440"))));
        assert_that!(&acc.get_iban().clone(), not(is(eq(String::from("ES32668805111122223334")))));
    }
}