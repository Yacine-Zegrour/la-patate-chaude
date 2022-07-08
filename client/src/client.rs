use crate::md5hash_cash;

#[test]
fn test_challenge() {

    assert_eq!(md5hash_cash(9,  "hello".to_string()), (844 as u64,"00441745D9BDF8E5D3C7872AC9DBB2C3".to_string()));
}