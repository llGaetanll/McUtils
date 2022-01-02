extern crate mc_utils;

use mc_utils::slime::SlimeMat;
use mc_utils::util::Point2D;

#[test]
fn test_max() {
    // load slime matrix
    let slime_mat = SlimeMat::load("s1.txt");

    println!("Slime Mat to string:\n{}", slime_mat);

    assert_eq!(slime_mat.p.x, -62);
    assert_eq!(slime_mat.p.z, -62);

    // find the most slime abundant 10x10 chunk area within the matrix
    let slime_perim = slime_mat.max_chunks(10, 10);

    assert_eq!(slime_perim.count, 22);
    assert_eq!(slime_perim.c1, Point2D{x: 121, z: 150});
    assert_eq!(slime_perim.c2, Point2D{x: 131, z: 160});

    assert_eq!(true, false);
}

#[test]
fn test_rank() {
    assert_eq!(true, true);
}
