extern crate mc_utils;

use mc_utils::util::{Groupable};

#[test]
fn test_space() {
    assert_eq!(vec![0, 0, 0, 0], 0.to_space());
    assert_eq!(vec![0, 0, 0, 63], 63.to_space());
    assert_eq!(vec![0, 0, 1, 0], 64.to_space());
    assert_eq!(vec![0, 0, 1, 36], 100.to_space());
    assert_eq!(vec![0, 0, 15, 40], 1000.to_space());
}

#[test]
fn test_time() {
    assert_eq!(vec![0, 0, 0, 0, 0, 0], 0.to_time());
    assert_eq!(vec![0, 0, 0, 0, 0, 59], 59.to_time());
    assert_eq!(vec![0, 0, 0, 0, 1, 0], 60.to_time());
    assert_eq!(vec![0, 0, 0, 0, 2, 10], 130.to_time());
    assert_eq!(vec![0, 0, 0, 0, 16, 40], 1_000.to_time());
    assert_eq!(vec![0, 0, 11, 13, 46, 40], 1_000_000.to_time());
    assert_eq!(vec![32, 1, 24, 1, 46, 40], 1_000_000_000.to_time());
}
