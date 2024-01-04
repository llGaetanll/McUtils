use crate::util::{Point3D, BlockPoint, FlatPoint, consts};

///
/// This module abstracts away in game commands into callable functions.
///


pub type Block = str;
pub type Command = String;
pub type CmdParams = Option<String>;

/**
    * returns a stringified setblock command given the block coordinate, type, and optional
    * parameters
    **/ 
pub fn setblock(p: &BlockPoint, block: &Block, params: CmdParams) -> Command {
    let params = params.unwrap_or("".to_string());

    format!("setblock {} {} {} {}{}", p.x, p.y, p.z, block, params)
}

/**
    * returns a stringified fill command given two block coordinates, a block type, and optional
    * parameters
    **/ 
pub fn fill(p1: &BlockPoint, p2: &BlockPoint, block: &Block, params: CmdParams) -> Command {
    let params = params.unwrap_or("".to_string());

    format!("fill {} {} {} {} {} {} {}{}", p1.x, p1.y, p1.z, p2.x, p2.y, p2.z, block, params)
}


/**
    * returns a set of commands to generate 1 high walls given a center block, a sidelength, and a
    * block type
    **/ 
pub fn walls_2d(c: &BlockPoint, sidelength: i32, block: &Block) -> Vec<Command> {
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
pub fn walls_3d(c: &FlatPoint<i32>, sidelength: i32, block: &Block) -> Vec<Command> {
    let mut cmds = Vec::new();

    let l = sidelength / 2;
    let verts = [
        FlatPoint {x: c.x - l, z: c.z - l},
        FlatPoint {x: c.x - l, z: c.z + l},
        FlatPoint {x: c.x + l, z: c.z - l},
        FlatPoint {x: c.x + l, z: c.z + l},
    ];

    for i in 0..verts.len() {
        // FIXME: bottom and top of the world have changed, use variables
        // NOTE: these values depend on the dimension
        cmds.push(fill(&verts[i].to_3d(0), &verts[(i + 1) % 4].to_3d(255), block, None));
    }

    cmds
}

/**
    * returns an escaped version of the passed in command, usable inside of another command block
    **/ 
fn esc(c: &Command) -> String {
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
        format!("{}{{id:command_block_minecart,Command:'{}'}},", s, esc(c))
    });

    // add one final command block to remove all previous command block minecarts in the chain
    passengers += "{{id:command_block_minecart,Command:'kill @e[type=minecraft:command_block_minecart,distance=..1]'}}";

    format!("summon falling_block ~ ~1 ~ {{Time:1,BlockState:{{Name:activator_rail}},Passengers:[{}]}}", passengers)
}
