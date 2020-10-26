use chrono::{NaiveDateTime};

pub struct Account{
    id: String,
    swift: String,
    iban:String,
    created_at: NaiveDateTime,
    status: bool,
}


impl Account{
    fn enable(){

    }

    fn disable(){

    }
    
}