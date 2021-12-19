use std::ops::Range;

fn main() {
    // target area: x=281..311, y=-74..-54
    let apex = find_highest_successful_arc_height((281..311, -74..-54), 100);
    dbg!(apex);
}

fn simulate_probe_arc(velocity: (i32, i32), max_steps: i32) -> Vec<(i32, i32)> {
    let mut positions = vec![];
    let mut position = (0, 0);
    let mut velocity = velocity;
    for _step in 0..max_steps {
        let (p, v) = model_probe_arc(position, velocity);
        positions.push(p);
        position = p;
        velocity = v;
    }
    positions
}

fn model_probe_arc(position: (i32, i32), velocity: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    (
        (position.0 + velocity.0, position.1 + velocity.1),
        ((velocity.0 - 1).max(0), velocity.1 - 1),
    )
}

fn check_probe_was_in_goal(arc: &[(i32, i32)], goal: &(Range<i32>, Range<i32>)) -> bool {
    // Update goal to be end inclusive
    let goal = (
        Range {
            start: goal.0.start,
            end: goal.0.end + 1,
        },
        Range {
            start: goal.1.start,
            end: goal.1.end + 1,
        },
    );

    for position in arc {
        if goal.0.contains(&position.0) && goal.1.contains(&position.1) {
            return true;
        }
    }
    false
}

fn get_apex_of_arc(arc: &[(i32, i32)]) -> i32 {
    let mut highest = 0;
    for position in arc {
        if position.1 > highest {
            highest = position.1;
        }
    }
    highest
}

fn find_highest_successful_arc_height(goal: (Range<i32>, Range<i32>), max_steps: i32) -> i32 {
    // fuzz values between x 0 and 10 and y -10 and 10
    let mut heights = vec![];
    for y in -1000..1000 {
        for x in -1000..1000 {
            let arc = simulate_probe_arc((x, y), max_steps);
            if check_probe_was_in_goal(&arc, &goal) {
                let apex = get_apex_of_arc(&arc);
                heights.push(apex);
            }
        }
    }

    *heights.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_if_a_probe_is_in_the_goal() {
        let arc = simulate_probe_arc((7, 2), 7);
        let result = check_probe_was_in_goal(&arc, &(20..30, -10..-5));
        assert_eq!(result, true);

        let arc = simulate_probe_arc((6, 3), 9);
        let result = check_probe_was_in_goal(&arc, &(20..30, -10..-5));
        assert_eq!(result, true);

        let arc = simulate_probe_arc((9, 0), 4);
        let result = check_probe_was_in_goal(&arc, &(20..30, -10..-5));
        assert_eq!(result, true);

        let arc = simulate_probe_arc((17, -4), 2);
        let result = check_probe_was_in_goal(&arc, &(20..30, -10..-5));
        assert_eq!(result, false);
    }

    #[test]
    fn it_finds_the_coolest_probe_shot() {
        let apex = find_highest_successful_arc_height((20..30, -10..-5), 20);
        assert_eq!(apex, 45);
    }
}