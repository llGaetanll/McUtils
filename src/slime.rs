use std::collections::BinaryHeap;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::io::BufReader;
use std::fs::File;
use std::env;
use std::fmt;

use regex::Regex;

use crate::util::Point2D;

type CMat = Vec<Vec<i32>>;

#[derive(Debug)]
pub struct SlimeMat {
    pub p: Point2D,
    pub seed: String, // java long
    pub mat: Vec<Vec<bool>>
}
// open file given filename
fn open(filename: &str) -> File {
    // gen absolute path
    let mut path = env::current_dir()
        .expect(&format!("Error getting current directory."));
    
    path.extend(["tests", "slime", filename]);

    // load file
    let file = File::open(&path)
        .expect(&format!("Error opening file {}.", path.display()));

    file
}

fn get_meta(meta_line: &str) -> (Point2D, String) {
    // extract fields
    let meta_regex = Regex::new(r"x: (-?\d+), z: (-?\d+), s: (-?\d+)").unwrap();
    let meta_fields = meta_regex.captures(&meta_line).unwrap();
    
    (
        Point2D {
            x: meta_fields.get(1).expect("x coordinate not found").as_str().parse()
                .expect("invalid coordinate"),

            z: meta_fields.get(2).expect("z coordinate not found").as_str().parse()
                .expect("invalid coordinate"),
        },
        meta_fields.get(3).expect("seed not found").as_str().parse()
                .expect("invalid seed")
    )
}

impl SlimeMat {
    /**
    * returns the cumulative matrix for the given slime matrix
    */
    fn to_c_mat(&self) -> CMat {
        let mat = &self.mat;
        let width = mat.len();
        let height = mat[0].len();

        // compute cumulative matrix
        let mut c_mat: CMat = vec![vec![0; height + 1]; width + 1];
        for i in 1..=width {
            for j in 1..=height {
                c_mat[i][j] = c_mat[i - 1][j] + c_mat[i][j - 1] - c_mat[i - 1][j - 1];

                if mat[i-1][j-1] {
                    c_mat[i][j] += 1;
                }
            }
        }

        c_mat
    }

    /**
    * returns the slime matrix struct for a given slime file.
    * The path should be absolute
    */
    pub fn buf_load(&self, filename: &str) -> SlimeMat {
        // size of buffer when reading text file
        const BUFSIZE: usize = 2048;

        // load file
        let file = open(filename);

        // create buffered reader
        let mut reader = BufReader::with_capacity(BUFSIZE, file);

        // read in metadata
        let mut meta_line = String::new();
        reader.read_line(&mut meta_line).unwrap();

        // extract metadata
        let (point, seed) = get_meta(&meta_line);

        // extract matrix
        // since this is buffered, it works with arbitrairily large matricies
        let mut matbuf = [0; BUFSIZE];

        let mut mat = Vec::new();
        let mut row = Vec::new();

        // read buffered matrix until nothing left
        while reader.read(&mut matbuf[..]).unwrap() != 0 {
            // read entire buffer
            for b in matbuf {
                // if we encouter a newline, update vectors
                if b as char == '\n' {
                    mat.push(row);
                    row = Vec::new();

                    continue;
                }

                row.push((b as char) == '1');
            }
        }

        // create struct
        let slime_mat = SlimeMat{
            p: point,
            seed,
            mat
        };

        slime_mat
    }

    /**
     * load slime matrix file line by line
     */
    pub fn load(filename: &str) -> SlimeMat {
        // load file
        let file = open(filename);

        let mut reader = BufReader::new(file);

        // read in metadata
        let mut meta_line = String::new();
        reader.read_line(&mut meta_line).unwrap();

        // extract metadata
        let (point, seed) = get_meta(&meta_line);

        // get matrix
        let mut mat = Vec::new();
        let mut row = Vec::new();

        // read matrix line by line
        for line in reader.lines() {
            let line = line.unwrap(); // ignore errors

            // read each char in line
            for c in line.chars() {
                row.push(c == '1');
            }

            mat.push(row);
            row = Vec::new();
        }

        // create struct
        let slime_mat = SlimeMat {
            p: point,
            seed,
            mat
        };

        slime_mat
    }

    /**
    * returns the absolute densest slime chunk area given a slime chunk matrix, a width, and a height
    */
    pub fn max_chunks(&self, width: usize, height: usize) -> SlimePerim {
        // compute cumulative matrix
        let c_mat = self.to_c_mat();

        // keep track of largest perim
        let mut perim = SlimePerim {
            count: -1,
            c1: Point2D { x: 0, z: 0 },
            c2: Point2D { x: 0, z: 0 },
        };

        // find most dense area
        for i in width + 1..c_mat.len() {
            for j in height + 1..c_mat.len() {
                let slime_count = c_mat[i][j] - c_mat[i - width][j] - c_mat[i][j - height] + c_mat[i - width][j - height];

                if slime_count > perim.count {
                    perim = SlimePerim {
                        count: slime_count,
                        c1: Point2D { x: (i - width) as i32 - self.p.x, z: (j - height) as i32 - self.p.z },
                        c2: Point2D { x: i as i32 - self.p.x, z: j as i32 - self.p.z }
                    };
                }
            }
        }

        perim
    }


    /**
    * returns a ranking of the densest slime chunk areas given a slime chunk matrix, a width, a height, a ranking size
    */
    pub fn max_chunk_rank(&self, width: usize, height: usize, size: usize) -> BinaryHeap<SlimePerim> {
        // compute cumulative matrix
        let c_mat = self.to_c_mat();

        // set up max heap
        let mut heap = BinaryHeap::with_capacity(size); // not sure if this replaces smaller ones with new ones

        for i in width + 1..c_mat.len() {
            for j in height + 1..c_mat.len() {
                let slime_count = c_mat[i][j] - c_mat[i - width][j] - c_mat[i][j - height] + c_mat[i - width][j - height];

                let perim = SlimePerim {
                    count: slime_count,
                    c1: Point2D { x: (i - width) as i32, z: (j - height) as i32 },
                    c2: Point2D { x: i as i32, z: j as i32 }
                };

                heap.push(perim);
            }
        }

        heap
    }

    /**
    * returns a strigified matrix alongside any metadata.
    */
    pub fn to_str(&self) -> String {
        let mut str_mat = format!("x: {}, z: {}, s: {}", &self.p.x, &self.p.z, &self.seed);

        let mat = &self.mat;
        for i in 0..mat.len() {
            let mut line = String::from("");

            for j in 0..mat[0].len() {
                let mut c = ' ';

                if mat[i][j] {
                    c = '#';
                }

                line.push(c);
            }

            str_mat = format!("{}\n{}", str_mat, line);
        }

        str_mat
    }
}

// printing a SlimeMat
impl fmt::Display for SlimeMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_mat = String::from("");

        let mat = &self.mat;
        for i in 0..mat.len() {
            let mut line = String::from("");

            for j in 0..mat[0].len() {
                let mut c = ' ';

                if mat[i][j] {
                    c = '#';
                }

                line.push(c);
            }

            str_mat = format!("{}\n{}", str_mat, line);
        }

        write!(f, "x: {}, z: {}, s: {}{}", &self.p.x, &self.p.z, &self.seed, str_mat)
    }
}


#[derive(Eq)]
pub struct SlimePerim {
    pub count: i32,

    // chunk coordinates
    pub c1: Point2D,
    pub c2: Point2D,
}

impl fmt::Display for SlimePerim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "count: {}, from: {}, to: {}", &self.count, &self.c1, &self.c2)
    }
}

impl Ord for SlimePerim {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for SlimePerim {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SlimePerim {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}
