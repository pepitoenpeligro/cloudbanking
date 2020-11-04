use crate::bankaccount::model::{Account};
use chrono::{NaiveDate, NaiveDateTime};
use crate::utils::model::{Lib};


#[cfg(test)]
/// Test for Bankaccount
mod test_bankaccount{
    use super::*;

    #[test]
    /// Test if ID assignment is correct (inmutable getter - constructor)
    fn test_get_id(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        assert_eq!(*acc.get_id(), String::from("507f1f77bcf86cd799439011"));
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

        assert_eq!(*acc.get_swift(), String::from("BSCHESMMXXX"));
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

        assert_eq!(*acc.get_iban(), String::from("ES32668805111122223334"));
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

        assert_eq!(*acc.get_date(), NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0));
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

        assert_eq!(*acc.get_status(), true);
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
        acc.set_swift(new_swift);
        assert_eq!(*acc.get_swift(), String::from("BBVAESMMXXX"));
        assert_ne!(*acc.get_swift(), String::from("BSCHESMMXXX"));
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
        assert_eq!(*acc.get_iban(), String::from("ES49998877666655554440"));
        assert_ne!(*acc.get_iban(), String::from("ES32668805111122223334"));
    }

}