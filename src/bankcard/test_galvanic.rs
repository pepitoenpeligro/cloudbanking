
mod test_bankcard_galvanic{
    use galvanic_assert::*;
    use galvanic_assert::matchers::*;

    use chrono::{NaiveDate, NaiveDateTime};
    use crate::bankcard::model::bankcard::*;


    #[test]
    /// Test if constructor is correct
    fn test_new_card(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let bc      : Card              = Card::new(id, card, cvc, date, status);

        assert_that!(&bc, is_variant!(Card));
    }

    #[test]
    /// Test if ID assignment is correct (inmutable getter - constructor)
    fn test_get_id(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let bc      : Card              = Card::new(id, card, cvc, date, status);

        assert_that!(bc.get_id(), eq(String::from("507f1f77bcf86cd799439011")));
    }

    #[test]
    /// Test if card_number assignment is correct (inmutable getter - constructor)
    fn test_get_card_number(){

        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let bc      : Card              = Card::new(id, card, cvc, date, status);

        assert_that!(&bc.get_number().clone(), eq(String::from("5555111122223333")));
    }

    #[test]
    /// Test if cvc assignment is correct (inmutable getter - constructor)
    fn test_get_cvc(){

        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let bc      : Card              = Card::new(id, card, cvc, date, status);

        assert_that!(&bc.get_cvc().clone(), eq(String::from("584")));
    }


    #[test]
    /// Test if date assignment is correct (inmutable getter - constructor)
    fn test_get_date(){

        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let bc      : Card              = Card::new(id, card, cvc, date, status);

        assert_that!(&bc.get_date().clone(), is(eq(NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0))));
    }

    #[test]
    /// Test if status assignment is correct (inmutable getter - constructor)
    fn test_get_status(){

        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let bc      : Card              = Card::new(id, card, cvc, date, status);

        assert_that!(&bc.is_active().clone(), is(eq(true)));
    }

    #[test]
    /// Test if swift assignment is correct (inmutable getter - setter)
    fn test_set_number(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let mut bc   : Card              = Card::new(id, card, cvc, date, status);


        let new_card  : String            = String::from("4444111122223333");
        bc.set_number(new_card.clone());
        assert_that!(&bc.get_number(), is(eq(String::from("4444111122223333"))));
        assert_that!(&bc.get_number(), not(is(eq(String::from("5555111122223333")))));
    }


    #[test]
    /// Test if swift assignment is correct (inmutable getter - setter)
    fn test_set_cvc(){
        let id      : String            = String::from("507f1f77bcf86cd799439011");
        let card    : String            = String::from("5555111122223333");
        let cvc     : String            = String::from("584");
        let date    : NaiveDateTime     = NaiveDate::from_ymd(2022, 7, 8).and_hms(22, 18, 0);
        let status  : bool              = true;
        let mut bc   : Card              = Card::new(id, card, cvc, date, status);


        let new_cvc  : String            = String::from("012");
        bc.set_cvc(new_cvc.clone());
        assert_that!(&bc.get_cvc(), is(eq(String::from("012"))));
        assert_that!(&bc.get_cvc(), not(is(eq(String::from("584")))));
    }

}