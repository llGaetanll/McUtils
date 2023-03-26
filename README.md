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


# Ideas

## Slime Module
These are the functions that I would like to build

- `private is_slime_chunk(seed, chunk_x, chunk_y) -> bool`: returns `True` if the chunk
  is a slime chunk.

- `top_p(seed, p, center, chunk_rad, search_rad) -> list?`: returns a descending
  list of the best places in the world to make a slime farm given a radius for
  search and for perimeter (both in chunks). Also returns `p` value.

  This should be multithreaded for max performance.

In there somewhere should be a function that prefers long patterns of slime
chunks since those farms are usually nicer to build. Maybe use a vertical filter
convolution?

## Witch module?
Now that we have `java_random` maybe this is manageable?

Could find potential locations for quad/triple witch huts in the world.

## Flower module?
Find the optimal location to build a flower forest farm minimizing for area.
