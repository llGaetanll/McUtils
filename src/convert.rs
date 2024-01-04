pub const STACK_SIZE: i32 = 64;
pub const SHULKER_SLOTS: i32 = CHEST_SLOTS;
pub const CHEST_SLOTS: i32 = 27;

//                                  full stack, full shulker, full DC of shulker
pub const SPACE_DENOM: [i32; 3] =   [STACK_SIZE, SHULKER_SLOTS, 2 * CHEST_SLOTS];
//                                  seconds, minutes, days, months, years
pub const TIME_DENOM: [i32; 5] =    [60, 60, 24, 30, 12];

pub trait Denomination {
    fn to_space(&self) -> String {
        todo!()
    }

    fn to_time(&self) -> String {
        todo!()
    }
}

/**
* returns a grouped collection (in to out) given a collection and a number n
*/
fn to_denom(col: Vec<i32>, n: i32) -> Vec<i32> {
    // create cumulative collection
    let mut cum_col = vec![];
    let mut t = 1;
    for c in col {
        t *= c;
        cum_col.push(t);
    }

    // reverse as to go from largest group to smallest
    cum_col.reverse();

    // iterate through cumulative groups to find remainders
    let mut grouped = vec![];
    let mut v = n;
    for cum_c in cum_col {
        grouped.push(v / cum_c);

        v %= cum_c;
    }

    // add final remainder
    grouped.push(v);

    grouped
}

impl Denomination for i32 {
    /**
    * returns a list representating the space necessary to store the number as a quantity of items
    **/
    fn to_space(&self) -> String {
        let units = ["item", "stack", "Shulker", "Shulker Double Chest"];
        let denom = to_denom(SPACE_DENOM.to_vec(), *self);

        let mut quants: Vec<String> = Vec::new();
        for (i, &quant) in denom.iter().rev().enumerate() {
            if quant == 0 {
                continue
            }

            let mut unit = units[i].to_owned();
            if quant != 1 {
                unit.push('s');
            }

            quants.push(format!("{} {}", quant, unit));
        }

        // edge case
        if quants.is_empty() {
            quants.push("0 items".to_owned());
        }

        quants.reverse();
        quants.join(", ")
    }

    /**
    * returns a list representating a time given a number of seconds
    **/
    fn to_time(&self) -> String {
        let units = ["second", "minute", "hour", "day", "month", "year"];
        let denom = to_denom(TIME_DENOM.to_vec(), *self);

        let mut quants: Vec<String> = Vec::new();
        for (i, &quant) in denom.iter().rev().enumerate() {
            if quant == 0 {
                continue
            }

            let mut unit = units[i].to_owned();
            if quant != 1 {
                unit.push('s');
            }

            quants.push(format!("{} {}", quant, unit));
        }

        // edge case
        if quants.is_empty() {
            quants.push("0 seconds".to_owned());
        }

        quants.reverse();
        quants.join(", ")
    }
}
