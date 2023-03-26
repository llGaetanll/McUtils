# McUtils

This library aims to provide various helpers for working with technical aspects
of Minecraft. Most of its functions come from personal needs and I would not
recommend that you use it as it lacks a lot of structure at the moment.

## Provided Helpers

- `util`: I don't know where else to put these functions
  - `to_denom`: convert a number to a given denomination
  - `to_space`: convert a number of items to
    - shulker Double Chests
    - shulkers
    - stacks
    - items
  - `to_time`: convert a duration in seconds to
    - years
    - months
    - days
    - hours
    - minutes
    - seconds
- `slime`: **TODO** reorganize
  - `max_chunk`: returns the densest area in a slime chunk matrix
  - `max_chunk_rank`: same but returns a ranking
- `nbt`: **TODO** basically nothing
- `cmd`: Generate command blocks command for the following functions.
  - `setblock`
  - `fill`
  - `walls_2d`: Generates 1 high walls
  - `walls_3d`: Generates walls from bottom to top of the world
  - `chain`: chains multiple commands into a single command block
