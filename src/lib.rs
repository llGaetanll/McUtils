//! This codebase contains different helpers for various Tech MC applications.
//!
//! Examples include:
//!  - Slime Chunk Finder
//!  - Structure Finders
//!  - ...
//!  - Command Generation & Command Chaining
//!  - Helper pseudo commands (walls, )

/**
 * Contains struct or trait definitions of the codebase.
 */
pub mod util;


/**
 * Contains helper functions to abstract away in-game commands
 */
pub mod cmd;

/**
 * Contains all functions dealing with slime chunks
 */
pub mod slime;

/**
 * Testing reading nbt
 */
pub mod nbt;
