use std::ops::Sub;
use std::fmt;

// block is just a string. Could do something more complicated but I would have to update it every
// new version. I made it a type in case I change my mind later.
pub type Block = str;

pub type Command = String;
pub type CmdParams = Option<String>;

// an item quanity can be represented as [# chests of shulkers, # shulkers, # stacks, # items]
// each enttry being the remainder of the last
// pub type Quantity = [i32; 4];

// ============================
//          CONSTANTS
// ============================
pub const CHUNK_SIZE: i32 = 16;
pub const WORLD_MAX: i32 = 256; // TODO: this is about to change soon!
pub const WORLD_MIN: i32 = 0;   // ...so is this

pub const STACK_SIZE: i32 = 64;
pub const SHULKER_SLOTS: i32 = CHEST_SLOTS;
pub const CHEST_SLOTS: i32 = 27;

//                                  full stack, full shulker, full DC of shulker
pub const SPACE_GROUP: [i32; 3] =   [STACK_SIZE, SHULKER_SLOTS, 2 * CHEST_SLOTS];
//                                  seconds, minutes, days, months, years
pub const TIME_GROUP: [i32; 5] =    [60, 60, 24, 30, 12];

// ============================
//           TRAITS
// ============================

pub trait Groupable {
    fn to_space(&self) -> Vec<i32>;
    fn to_time(&self) -> Vec<i32>;
}

/**
* returns a grouped collection (in to out) given a collection and a number n
*/
fn to_group(col: Vec<i32>, n: i32) -> Vec<i32> {
    // create cumulative collection
    let mut cum_col = vec![];
    let mut t = 1;
    for c in col {
        t = t * c;
        cum_col.push(t);
    }

    // reverse as to go from largest group to smallest
    cum_col.reverse();

    // iterate through cumulative groups to find remainders
    let mut grouped = vec![];
    let mut v = n;
    for cum_c in cum_col {
        grouped.push(v / cum_c);

        v = v % cum_c;
    }

    // add final remainder
    grouped.push(v);

    grouped
}

impl Groupable for i32 {
    /**
    * returns a list representating the space necessary to store the number as a quantity of items
    **/
    fn to_space(&self) -> Vec<i32> {
        to_group(SPACE_GROUP.to_vec(), *self)
    }

    /**
    * returns a list representating a time given a number of seconds
    **/
    fn to_time(&self) -> Vec<i32> {
        to_group(TIME_GROUP.to_vec(), *self)
    }
}

// ============================
//           STRUCTS
// ============================

#[derive(Eq, Debug)]
pub struct Point2D {
    // max x and z values are ~30 million either way so this is plenty
    pub x: i32, 
    pub z: i32
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z
    }
}

impl Sub for Point2D {
    type Output = Self;

    // takes itself and another struct of TYPE self
    fn sub(self, other: Self) -> Self::Output {
        Point2D {
            x: self.x - other.x,
            z: self.z - other.z,
        }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.z)
    }
}

impl Point2D {
    /**
    * returns a Point3D from a Point2D given a y coordinate
    **/
    pub fn to_3d(&self, y: i32) -> Point3D {
        Point3D { x: self.x, y: y, z: self.z }
    }

    /**
    * returns the euclidean distance from the current point given another Point2D
    **/
    pub fn dist(&self, p: Point2D) -> u32 {
        (((self.x - p.x).pow(2) + (self.z - p.z).pow(2)) as f64).sqrt() as u32
    }

    /**
    * Converts from block coordinate to chunk coordinate
    * returns the smallest coordinate point in the same chunk as the given point
    **/
    pub fn as_chunk(&self) -> Point2D {
        Point2D {x: (self.x / CHUNK_SIZE) * CHUNK_SIZE, z: (self.z / CHUNK_SIZE) * CHUNK_SIZE }
    }

    /**
    * Converts from chunk coordinate to block coordinate
    **/
    pub fn as_coord(&self) -> Point2D {
        Point2D {x: self.x * CHUNK_SIZE, z: self.z * CHUNK_SIZE }
    }

    pub fn to_str(&self) -> String {
        format!("({}, {})", self.x, self.z)
    }
}

#[derive(Eq, Debug)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point3D {
    /**
    * Cast a Point3D to Point2D
    **/
    pub fn to_2d(&self) -> Point2D {
        Point2D { x: self.x, z: self.z }
    }

    /**
    * Compute the euclidean distance between 2 3D Points
    **/
    pub fn dist(&self, p: Point3D) -> u32 {
        (((self.x - p.x).pow(2) + ((self.y - p.y) as i32).pow(2) + (self.z - p.z).pow(2)) as f64).sqrt() as u32
    }

    /**
    * Converts from block coordinate to chunk coordinate
    * returns the smallest coordinate point in the same chunk as the given point
    **/
    pub fn as_chunk(&self) -> Point3D {
        self.to_2d().as_chunk().to_3d(self.y)
    }

    /**
    * Converts from chunk coordinate to block coordinate
    **/
    pub fn as_coord(&self) -> Point3D {
        self.to_2d().as_coord().to_3d(self.y)
    }

    pub fn to_str(&self) -> String {
        format!("({}, {})", self.x, self.z)
    }
}

