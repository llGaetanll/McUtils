extern crate mc_utils;

use std::collections::HashMap;

use mc_utils::util::{Denominable};

#[test]
fn test_space() {
    let tests: HashMap<i32, &str> = vec![
        (0, "0 items"),
        (63, "63 items"),
        (64, "1 stack"),
        (130, "2 stacks, 2 items"),
        (1_728, "1 Shulker"),
        (93_312, "1 Shulker Double Chest"),
    ]
    .into_iter()
    .collect();

    for (input, output) in tests {
        assert_eq!(output, input.to_space())
    }
}

#[test]
fn test_time() {
    let tests: HashMap<i32, &str> = vec![
        (0, "0 seconds"),
        (59, "59 seconds"),
        (60, "1 minute"),
        (130, "2 minutes, 10 seconds"),
        (1_000, "16 minutes, 40 seconds"),
        (1_000_000, "11 days, 13 hours, 46 minutes, 40 seconds"),
        (1_000_000_000, "32 years, 1 month, 24 days, 1 hour, 46 minutes, 40 seconds")
    ]
    .into_iter()
    .collect();

    for (input, output) in tests {
        assert_eq!(output, input.to_time())
    }
}
