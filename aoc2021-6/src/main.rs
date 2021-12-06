use std::collections::HashMap;
use std::fs;

fn main() {
    let initial_state: Vec<u8> = fs::read_to_string("input")
        .expect("couldn't read the input file")
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect();

    let lake = fish_simulator(&initial_state, 80);
    dbg!(lake.into_values().sum::<u64>());

    let lake = fish_simulator(&initial_state, 256);
    dbg!(lake.into_values().sum::<u64>());
}

fn fish_simulator(input: &[u8], duration: u32) -> HashMap<u8, u64> {
    // Create the empty lake
    let mut lake: HashMap<u8, u64> = HashMap::new();
    for count in 0..9 {
        lake.insert(count, 0);
    }

    // Update lake with initial state
    for fish in input {
        if let Some(count) = lake.get_mut(fish) {
            *count += 1;
        }
    }

    // Run the simulation
    for _day in 0..duration {
        // Create tomorrow's lake
        let mut next_lake: HashMap<u8, u64> = HashMap::new();
        for timer in 0..9 {
            next_lake.insert(timer, 0);
        }

        // For each set of fish with the same timer value...
        let mut timers: Vec<u8> = lake.clone().into_keys().collect();
        timers.sort_unstable();
        timers.reverse();
        for timer in timers {
            if let Some(&count) = lake.get(&timer) {
                // Is it time to spawn a new fish yet?
                if timer > 0 {
                    // Not yet
                    next_lake.insert(timer - 1, count);
                } else {
                    // New mom is in the delivery room
                    if let Some(new_moms) = next_lake.get_mut(&6) {
                        *new_moms += count;
                    }

                    // A new fish is born!
                    if let Some(newborns) = next_lake.get_mut(&8) {
                        *newborns += count;
                    }
                }
            }
        }

        // Tomorrow's lake is ready
        lake = next_lake;
    }

    lake
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_returns_the_initial_state_of_the_pond() {
        let input = "3,4,3,1,2";
        let initial_state: Vec<u8> = input
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();

        let end_state: HashMap<u8, u64> = fish_simulator(&initial_state, 0);
        assert_eq!(end_state.get(&1), Some(&1));
        assert_eq!(end_state.get(&2), Some(&1));
        assert_eq!(end_state.get(&3), Some(&2));
        assert_eq!(end_state.get(&4), Some(&1));
    }

    #[test]
    fn it_calculates_the_number_of_fish_after_2_days() {
        let input = "3,4,3,1,2";
        let initial_state: Vec<u8> = input
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();

        let end_state = fish_simulator(&initial_state, 2);
        assert_eq!(end_state.get(&0), Some(&1));
        assert_eq!(end_state.get(&1), Some(&2));
        assert_eq!(end_state.get(&2), Some(&1));
        assert_eq!(end_state.get(&6), Some(&1));
        assert_eq!(end_state.get(&8), Some(&1));
    }

    #[test]
    fn it_calculates_the_number_of_fish_after_18_days() {
        let input = "3,4,3,1,2";
        let initial_state: Vec<u8> = input
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();

        let end_state = fish_simulator(&initial_state, 18);
        assert_eq!(end_state.into_values().sum::<u64>(), 26);
    }

    #[test]
    fn it_calculates_the_number_of_fish_after_256_days() {
        let input = "3,4,3,1,2";
        let initial_state: Vec<u8> = input
            .split(",")
            .map(|value| value.parse().unwrap())
            .collect();

        let end_state = fish_simulator(&initial_state, 256);
        assert_eq!(end_state.into_values().sum::<u64>(), 26984457539);
    }
}
