mod test_bankfund_galvanic{
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;

    use chrono::{NaiveDate, NaiveDateTime};
    use crate::bankfund::model::bankfund::*;


    #[test]
    /// Test if constructor is correct
    fn test_new_fund(){
        let id          : String            = String::from("507f1f77bcf86cd799439011");
        let amount      : f64               = 534.4;
        let date_start  : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let date_finish : NaiveDateTime     = NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0);
        let status      : bool              = true;

        let bf          : Fund                = Fund::new(id, amount,date_start,date_finish,status);

        assert_that!(&bf, is_variant!(Fund));
    }

    #[test]
    /// Test if ID assignment is correct (inmutable getter - constructor)
    fn test_get_id(){
        let id          : String            = String::from("507f1f77bcf86cd799439011");
        let amount      : f64               = 534.4;
        let date_start  : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let date_finish : NaiveDateTime     = NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0);
        let status      : bool              = true;

        let bf          : Fund                = Fund::new(id, amount,date_start,date_finish,status);

        assert_that!(bf.get_id(), eq(String::from("507f1f77bcf86cd799439011")));
    }


    #[test]
    /// Test if card_number assignment is correct (inmutable getter - constructor)
    fn test_get_amount(){

        let id          : String            = String::from("507f1f77bcf86cd799439011");
        let amount      : f64               = 534.4;
        let date_start  : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let date_finish : NaiveDateTime     = NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0);
        let status      : bool              = true;

        let bf          : Fund                = Fund::new(id, amount,date_start,date_finish,status);

        assert_that!(&bf.get_amount().clone(), eq(534.4));
    }


    #[test]
    /// Test if date assignment is correct (inmutable getter - constructor)
    fn test_get_start_date(){

        let id          : String            = String::from("507f1f77bcf86cd799439011");
        let amount      : f64               = 534.4;
        let date_start  : NaiveDateTime     = NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0);
        let date_finish : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status      : bool              = true;

        let bf          : Fund                = Fund::new(id, amount,date_start,date_finish,status);

        assert_that!(&bf.get_start_date().clone(), is(eq(NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0))));
    }


    #[test]
    /// Test if date assignment is correct (inmutable getter - constructor)
    fn test_get_finish_date(){

        let id          : String            = String::from("507f1f77bcf86cd799439011");
        let amount      : f64               = 534.4;
        let date_start  : NaiveDateTime     = NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0);
        let date_finish : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status      : bool              = true;

        let bf          : Fund                = Fund::new(id, amount,date_start,date_finish,status);

        assert_that!(&bf.get_finish_date().clone(), is(eq(NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0))));
    }


    #[test]
    /// Test if status assignment is correct (inmutable getter - constructor)
    fn test_get_status(){

        let id          : String            = String::from("507f1f77bcf86cd799439011");
        let amount      : f64               = 534.4;
        let date_start  : NaiveDateTime     = NaiveDate::from_ymd(2025, 7, 8).and_hms(22, 18, 0);
        let date_finish : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status      : bool              = true;

        let bf          : Fund                = Fund::new(id, amount,date_start,date_finish,status);

        assert_that!(&bf.is_active().clone(), is(eq(true)));
    }



}