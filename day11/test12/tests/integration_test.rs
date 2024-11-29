
use test12::*;


mod common;

#[test]
fn it_works() {
    common::setup();
    assert_eq!(add(2, 2),4);
}


