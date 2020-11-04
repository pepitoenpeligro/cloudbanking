mod test_user{
    use super::*;
    use std::collections::HashMap;
    use crate::utils::model::{Lib};
    use chrono::{NaiveDateTime,NaiveDate};
    use crate::bankaccount::model::bankaccount::{Account};
    use crate::user::model::User::{User};



    #[test]
    /// Test add_bank_account
    /// Check if a bank_account is added to bank_accounts Collection
    fn test_add_bank_account(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        let id_user             : String                    = String::from("999f7f66abf88ee70243988");
        let email_user          : String                    = String::from("j.cordoba@ostfalia.de");
        let hash_password_use   : String                    = String::from("aa4234bdsfasf");
        let date_user           : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let mut user                : User                      = User::new(id_user,email_user,hash_password_use,date_user);


        assert_eq!(user.get_bank_accounts().len(),0);
        assert_eq!(user.get_bank_accounts().contains_key("507f1f77bcf86cd799439011"), false);
        
        user.add_bank_account(acc);

        assert_eq!(user.get_bank_accounts().contains_key("507f1f77bcf86cd799439011"), true);
        assert_eq!(user.get_bank_accounts().len(),1);
    }

}