/// This module contains any random utility.
///
/// In it, you'll find functions for computing
/// - slime chunks
/// - flower type generation
/// - bedrock generation patterns

///
/// Contains all functions dealing with slime chunks.
///
pub mod slime;

///
/// Contains helpers for working with flowers in flower forest biomes.
///
pub mod flowers;


///
/// Contains utilities for predicting bedrock generation patterns.
/// 
pub mod bedrock;
pub mod bedrock_new;

// re-exports
pub use slime::is_slimechunk;

pub use flowers::flower_at;
pub use flowers::FlowerForestFlower;
