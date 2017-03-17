extern crate veye_checker;

use veye_checker::api;

#[test]
fn test_encoding_product_key(){
    assert_eq!(api::encode_prod_key("dot.net"), "dot~net");
    assert_eq!(api::encode_prod_key("slash/net"), "slash:net");
    assert_eq!(api::encode_prod_key("dot.net/slash"), "dot~net:slash");
}