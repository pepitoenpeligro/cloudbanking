use crate::utils::model::{Lib};

mod test_utils{
    use super::*;
    #[test]
    /// Test if a swift code is valid or not
    fn test_check_valid_swift(){
        let new_swift   : String            = String::from("BBVAESMMXXX");
        let bad_swift   : String            = String::from("BVAEMMX");

        assert_eq!(Lib::check_swift(new_swift), true);
        assert_eq!(Lib::check_swift(bad_swift), false);
    }


    #[test]
    /// Test if a swift code is valid or not
    fn test_check_valid_iban(){
        let new_iban   : String            = String::from("ES49998877666655554440");
        let german_iban   : String         = String::from("DE63250500000201414513");
        let bad_iban   : String            = String::from("XXX632505000002014145");

        assert_eq!(Lib::check_iban(new_iban), true);
        assert_eq!(Lib::check_iban(german_iban), true);
        assert_eq!(Lib::check_iban(bad_iban), false);
    }
}