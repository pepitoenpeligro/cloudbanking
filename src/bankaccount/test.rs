#[cfg(test)]
mod test_bankaccount{
    use super::*;

    #[test]
    // This test check if a BankAccount can set name
    fn test_name(){
        let to_expect = "Jose";
        let bankAccount = "{\"name\" : \"Jose Antonio\"}";
        assert!(bankAccount.contains(to_expect));
    }
}