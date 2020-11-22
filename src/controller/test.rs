mod test_galvanic_controller{
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::collection::*;

    use chrono::{NaiveDate, NaiveDateTime};
    use crate::bankaccount::model::bankaccount::*;
    use crate::user::model::user::User;
    use crate::bankcard::model::bankcard::*;
    use crate::utils::model::{Lib};
    use crate::controller::model::{CloudBankingController};

    #[test]
    /// Check if user is created in System
    /// HU 14
    fn test_create_new_user(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);
    
        let id_user             : String                    = String::from("999f7f66abf88ee70243988");
        let email_user          : String                    = String::from("j.cordoba@ostfalia.de");
        let date_user           : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let user                : User                      = User::new(id_user,email_user,date_user);

        let mut controller: CloudBankingController = CloudBankingController::new();

        // At this point we don't expect found this user
        assert_that!(&controller.get_users().read().unwrap().contains_key("999f7f66abf88ee70243988"), is(eq(false)));
        controller.create_new_user(user);

        // At this point we EXPECT found this user
        assert_that!(&controller.get_users().read().unwrap().contains_key("999f7f66abf88ee70243988"), is(eq(true)));

    }


    #[test]
    /// Check if erased from created in System
    /// Check HU 16
    fn test_erase_account(){

        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let swift   : String            = String::from("BSCHESMMXXX");
        let iban    : String            = String::from("ES32668805111122223334");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let acc     : Account           = Account::new(id, swift, iban, date, status);
    
        let id_user             : String                    = String::from("999f7f66abf88ee70243988");
        let email_user          : String                    = String::from("j.cordoba@ostfalia.de");
        let date_user           : NaiveDateTime             = NaiveDate::from_ymd(2020, 7, 8).and_hms(22, 18, 0);
        let user                : User                      = User::new(id_user,email_user,date_user);

        let mut controller: CloudBankingController = CloudBankingController::new();
        // At this point we don't expect found this user
        assert_that!(&controller.get_users().read().unwrap().contains_key("999f7f66abf88ee70243988"), is(eq(false)));
        controller.create_new_user(user);

        // At this point we EXPECT found this user
        assert_that!(&controller.get_users().read().unwrap().contains_key("999f7f66abf88ee70243988"), is(eq(true)));
        controller.erase_account(String::from("999f7f66abf88ee70243988"));


        // At this point we EXPECT found this user
        assert_that!(&controller.get_users().read().unwrap().contains_key("999f7f66abf88ee70243988"), is(eq(false)));

    }
}
