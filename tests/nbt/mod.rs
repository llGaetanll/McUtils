extern crate mc_utils;

use std::fs::File;
use mc_utils::nbt::disp;

#[test]
fn test_disp() {
    let ex1 = File::open("tests/nbt/ex1.litematic").unwrap();
    disp(ex1);

    assert_eq!(false, true);
}


