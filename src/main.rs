/**
 *
 * This codebase contains different helpers for various Tech MC applications.
 *
 * Examples include:
 *  - Slime Chunk Finder
 *  - Structure Finders
 *  - ...
 *  - Command Generation & Command Chaining
 *  - Helper pseudo commands (walls, )
 *
 **/
fn main() {
    println!("Hello, world!");
}

mod util {
    // ============================
    //           TRAITS
    // ============================

    pub trait Countable {
        fn to_space(&self) -> Vec<i32>;
        fn to_time(&self) -> Vec<i32>;
    }

    // ============================
    //           STRUCTS
    // ============================

    pub struct Point2D {
        // max x and z values are ~30 million either way so this is plenty
        pub x: i32, 
        pub z: i32
    }

    impl Point2D {
        /**
         * returns a Point3D from a Point2D given a y coordinate
         **/
        pub fn to_3D(&self, y: i32) -> Point3D {
            Point3D { x: self.x, y: y, z: self.z }
        }

        /**
         * returns the euclidean distance from the current point given another Point2D
         **/
        pub fn dist(&self, p: Point2D) -> u32 {
            (((self.x - p.x).pow(2) + (self.z - p.z).pow(2)) as f64).sqrt() as u32
        }

        /**
         * returns the smallest coordinate point in the same chunk as the given point
         **/
        pub fn to_chunk_bdn(&self) -> Point2D {
            Point2D {x: (self.x / CHUNK_SIZE) * CHUNK_SIZE, z: (self.z / CHUNK_SIZE) * CHUNK_SIZE }
        }
    }

    pub struct Point3D {
        pub x: i32,
        pub y: i32,
        pub z: i32
    }

    impl Point3D {
        /**
         * Cast a Point3D to Point2D
         **/
        pub fn to_2D(&self) -> Point2D {
            Point2D { x: self.x, z: self.z }
        }

        /**
         * Compute the euclidean distance between 2 3D Points
         **/
        pub fn dist(&self, p: Point3D) -> u32 {
            (((self.x - p.x).pow(2) + ((self.y - p.y) as i32).pow(2) + (self.z - p.z).pow(2)) as f64).sqrt() as u32
        }

        /**
         * returns the smallest point coordinate in the same chunk as the given point. The Y is kept the
         * same
         **/
        pub fn to_chunk_bdn(&self) -> Point3D {
            self.to_2D().to_chunk_bdn().to_3D(self.y)
        }
    }

    // ============================
    //          FUNCTIONS
    // ============================

    /**
    * returns a grouped collection (in to out) given a collection and a number n
    */
    pub fn to_col_group(col: Vec<i32>, n: i32) -> Vec<i32> {
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

    // ============================
    //            TYPES
    // ============================

    // block is just a string. Could do something more complicated but I would have to update it every
    // new version. I made it a type in case I change my mind later.
    pub type Block = str;

    pub type Command = String;
    pub type CmdParams = Option<String>;

    // an item quanity can be represented as [# chests of shulkers, # shulkers, # stacks, # items]
    // each enttry being the remainder of the last
    pub type Quantity = [i32; 4];

    // ============================
    //          CONSTANTS
    // ============================
    const CHUNK_SIZE: i32 = 16;
    const WORLD_MAX: i32 = 256; // TODO: this is about to change soon!
    const WORLD_MIN: i32 = 0; // ...so is this

    pub const STACK_SIZE: i32 = 64;
    pub const SHULKER_SLOTS: i32 = CHEST_SLOTS;
    pub const CHEST_SLOTS: i32 = 27;
    // const DOUBLE_CHEST_SLOTS: i32 = 2 * CHEST_SLOTS;
}

/**
 * returns a stringified version of the command
 **/
mod cmd_str {
    // for now import every type
    use crate::util::*;

    /**
     * returns a stringified setblock command given the block coordinate, type, and optional
     * parameters
     **/ 
    pub fn setblock(p: &Point3D, block: &Block, params: CmdParams) -> Command {
        let params = params.unwrap_or("".to_string());

        format!("setblock {} {} {} {}{}", p.x, p.y, p.z, block, params)
    }

    /**
     * returns a stringified fill command given two block coordinates, a block type, and optional
     * parameters
     **/ 
    pub fn fill(p1: &Point3D, p2: &Point3D, block: &Block, params: CmdParams) -> Command {
        let params = params.unwrap_or("".to_string());

        format!("fill {} {} {} {} {} {} {}{}", p1.x, p1.y, p1.z, p2.x, p2.y, p2.z, block, params)
    }


    /**
     * returns a set of commands to generate 1 high walls given a center block, a sidelength, and a
     * block type
     **/ 
    pub fn walls2D(c: &Point3D, sidelength: i32, block: &Block) -> Vec<Command> {
        let mut cmds = Vec::new();

        let l = sidelength / 2;
        let verts = [
            Point3D {x: c.x - l, y: c.y, z: c.z - l},
            Point3D {x: c.x - l, y: c.y, z: c.z + l},
            Point3D {x: c.x + l, y: c.y, z: c.z - l},
            Point3D {x: c.x + l, y: c.y, z: c.z + l},
        ];

        for i in 0..verts.len() {
            cmds.push(fill(&verts[i], &verts[(i + 1) % 4], block, None));
        }

        cmds
    }

    /**
     * returns a set of commands to generate walls given a center block, a sidelength, and a
     * block type. The walls extend from the bottom to the top of the world
     **/ 
    pub fn walls3D(c: &Point2D, sidelength: i32, block: &Block) -> Vec<Command> {
        let mut cmds = Vec::new();

        let l = sidelength / 2;
        let verts = [
            Point2D {x: c.x - l, z: c.z - l},
            Point2D {x: c.x - l, z: c.z + l},
            Point2D {x: c.x + l, z: c.z - l},
            Point2D {x: c.x + l, z: c.z + l},
        ];

        for i in 0..verts.len() {
            cmds.push(fill(&verts[i].to_3D(0), &verts[(i + 1) % 4].to_3D(255), block, None));
        }

        cmds
    }

    /**
     * returns an escaped version of the passed in command, usable inside of another command block
     **/ 
    fn esc(c: Command) -> String {
        c
            .replace("\"", "\\\"")  // " -> \"
            .replace("'", "\\'")    // ' -> \'
            .replace("\\", "\\\\")  // \ -> \\
    }

    /**
     * returns a single command to execute every command in the given list in order
     **/ 
    pub fn chain(cmds: Vec<Command>) -> Command {
        // create a list of passenger command blocks minecarts from escaped commands
        let mut passengers = cmds.iter().fold("".to_string(), |s, c| { 
            s + format!("{{id:command_block_minecart,Command:'{}'}},", esc(*c)) 
        });

        // add one final command block to remove all previous command block minecarts in the chain
        passengers += "{{id:command_block_minecart,Command:'kill @e[type=minecraft:command_block_minecart,distance=..1]'}}";

        format!("summon falling_block ~ ~1 ~ {{Time:1,BlockState:{{Name:activator_rail}},Passengers:[{}]}}", passengers)
    }

    /**
     * returns a single command to execute every chain of command in the list in order
     **/ 
    pub fn group(cmds: Vec<Vec<Command>>) -> Command {
        "unimplemented".to_string()
    }
}

mod helpers {
    use crate::util::*;

    impl Countable for i32 {
        /**
         * returns a list representating the space necessary to store the number as a quantity of items
         **/
        fn to_space(&self) -> Vec<i32> {
            // define item group
            let item_col = vec![STACK_SIZE, SHULKER_SLOTS, 2 * CHEST_SLOTS];

            to_col_group(item_col, *self)
        }

        /**
         * returns a list representating a time given a number of seconds
         **/
        fn to_time(&self) -> Vec<i32> {
            // define item group
            let time_col = vec![60, 60, 24, 365];

            to_col_group(time_col, *self)
        }
    }
}
