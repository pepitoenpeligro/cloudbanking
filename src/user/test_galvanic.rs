/// Module test_galvanic_user
/// Check all functionality of use

mod test_galvanic_user{
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::collection::*;

    use chrono::{NaiveDate, NaiveDateTime};
    use crate::bankaccount::model::bankaccount::*;
    use crate::user::model::user::User;
    use crate::bankcard::model::{Card};
    use crate::utils::model::{Lib};

    #[test]
    /// Scenario HU1
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
        let date_user           : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let mut user            : User                      = User::new(id_user,email_user,date_user);


        assert_that!(&acc,is_variant!(Account));
        assert_that!(&user, is_variant!(User));

        assert_that!(&(user.get_bank_accounts().len() as i32) ,is(eq(0 as i32)));
        assert_that!(&user.get_bank_accounts().contains_key("507f1f77bcf86cd799439011"), is(eq(false)));
        
        user.add_bank_account(acc);

        assert_that!(&user.get_bank_accounts().contains_key("507f1f77bcf86cd799439011"), is(eq(true)));
        assert_that!(&(user.get_bank_accounts().len() as i32) ,is(eq(1 as i32)));
    }


    #[test]
    /// Scenario HU5
    /// Test delete_bank_account
    /// Check if a bank_account is removed from bank_accounts Collection
    fn test_delete_bank_account(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);

        let id_user             : String                    = String::from("999f7f66abf88ee70243988");
        let email_user          : String                    = String::from("j.cordoba@ostfalia.de");
        let date_user           : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let mut user            : User                      = User::new(id_user,email_user,date_user);


        assert_that!(&acc,is_variant!(Account));
        assert_that!(&user, is_variant!(User));

        assert_that!(&(user.get_bank_accounts().len() as i32) ,is(eq(0 as i32)));
        assert_that!(&user.get_bank_accounts().contains_key("507f1f77bcf86cd799439011"), is(eq(false)));
        
        user.add_bank_account(acc);

        assert_that!(&user.get_bank_accounts().contains_key("507f1f77bcf86cd799439011"), is(eq(true)));
        assert_that!(&(user.get_bank_accounts().len() as i32) ,is(eq(1 as i32)));
        
        assert_that!(&user.get_bank_accounts().contains_key(&String::from("507f1f77bcf86cd799439011")), is(eq(true)));
        user.delete_bank_account(String::from("507f1f77bcf86cd799439011"));
        assert_that!(&user.get_bank_accounts().contains_key(&String::from("507f1f77bcf86cd799439011")), not(is(eq(true))));
    }
}




