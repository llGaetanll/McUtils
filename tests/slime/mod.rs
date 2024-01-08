use mc_utils::rand::is_slimechunk;
use mc_utils::rand::slime::is_slimechunk_emma;

use ndarray::Array1;
use ndarray::Array2;

use regex::Regex;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::Result;
use std::path::PathBuf;

#[derive(Debug)]
struct SlimeMat {
    // the x chunk coordinate of the top left of the file
    x: i32,

    // the z chunk coordinate of the top left of the file
    z: i32,

    seed: i64,

    slime_chunks: Array2<bool>,
}

fn load_file(name: &str) -> io::Result<SlimeMat> {
    let path: PathBuf = ["tests", "slime", name].iter().collect();

    let file = fs::read_to_string(path)?;
    let mut lines = file.lines();

    let meta_line = lines
        .next()
        .ok_or(io::Error::new(ErrorKind::Other, "missing meta line"))?;
    let meta_regex = Regex::new(r"x: (-?\d+), z: (-?\d+), s: (-?\d+)").unwrap();
    let meta_fields = meta_regex.captures(meta_line).unwrap();

    let x: i32 = meta_fields
        .get(1)
        .expect("x coordinate not found")
        .as_str()
        .parse()
        .expect("invalid coordinate");

    let z: i32 = meta_fields
        .get(2)
        .expect("z coordinate not found")
        .as_str()
        .parse()
        .expect("invalid coordinate");

    let seed: i64 = meta_fields
        .get(3)
        .expect("seed not found")
        .as_str()
        .parse()
        .expect("invalid seed");

    let (width, height) = {
        let mut lines = lines.clone();
        let width = lines.next().unwrap().len();
        let height = lines.count() + 1; // since we consumed a line getting the width

        (width, height)
    };

    let slime_chunks = lines.flat_map(|line| {
        line.chars().map(|c| match c {
            '1' => true,
            '0' => false,
            _ => panic!("invalid slime chunk matrix"),
        })
    });

    let slime_chunks: Array2<bool> = Array1::from_iter(slime_chunks)
        .into_shape((width, height))
        .unwrap();

    Ok(SlimeMat { x, z, seed, slime_chunks })
}

#[test]
fn slime_chunk_s1() -> Result<()> {
    let mat = load_file("s1.txt")?;
    let (x, z, seed) = (mat.x, mat.z, mat.seed);

    mat.slime_chunks.indexed_iter().for_each(|((dz, dx), value)| {
        let (x, z) = (x + dx as i32, z + dz as i32);

        let res = is_slimechunk_emma(seed, x, z);
        assert_eq!(*value, res, "[s1.txt] is_slimechunk({seed}, {x}, {z}). Expected: {value}. Got {res}")
    });

    Ok(())
}

#[test]
fn slime_chunk_s2() -> Result<()> {
    let mat = load_file("s2.txt")?;
    let (x, z, seed) = (mat.x, mat.z, mat.seed);

    mat.slime_chunks.indexed_iter().for_each(|((dz, dx), value)| {
        let (x, z) = (x + dx as i32, z + dz as i32);

        let res = is_slimechunk_emma(seed, x, z);
        assert_eq!(*value, res, "[s2.txt] is_slimechunk({seed}, {x}, {z}). Expected: {value}. Got {res}")
    });

    Ok(())
}

#[test]
fn slime_chunk_s3() -> Result<()> {
    let mat = load_file("s3.txt")?;
    let (x, z, seed) = (mat.x, mat.z, mat.seed);

    mat.slime_chunks.indexed_iter().for_each(|((dz, dx), value)| {
        let (x, z) = (x + dx as i32, z + dz as i32);

        let res = is_slimechunk_emma(seed, x, z);
        assert_eq!(*value, res, "[s3.txt] is_slimechunk({seed}, {x}, {z}). Expected: {value}. Got {res}")
    });

    Ok(())
}
