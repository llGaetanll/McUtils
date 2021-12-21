extern crate mc_utils;

use std::fs::File;
use std::path::Path;

use mc_utils::slime::*;

fn load_slime_file(path: Path) -> SlimeMat {
    let mut file = File::open(&path).expect(format!("Error opening file {}.", path.display()));

    // read metadata line


    let slime_mat = SlimeMat{

    }

    slime_mat
}

#[test]
fn test_max() {
    assert_eq!(true, true);
}

#[test]
fn test_rank() {
    assert_eq!(true, true);
}
