use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

struct Position {
    x: i32,
    y: i32,
}

struct Velocity {
    x: i32,
    y: i32,
}

fn simulate_probe_arc(velocity: Velocity, steps: i32) -> Position {
    let mut position = Position { x: 0, y: 0 };
    let mut velocity = velocity;
    for _step in 0..steps {
        let (p, v) = model_probe_arc(position, velocity);
        position = p;
        velocity = v;
    }
    position
}

fn model_probe_arc(position: Position, velocity: Velocity) -> (Position, Velocity) {
    (
        Position {
            x: position.x + velocity.x,
            y: position.y + velocity.y,
        },
        Velocity {
            x: (velocity.x - 1).max(0),
            y: velocity.y - 1,
        },
    )
}

fn check_probe_is_in_goal(probe_position: &Position, goal: (Range<i32>, Range<i32>)) -> bool {
    goal.0.contains(&probe_position.x) && goal.1.contains(&probe_position.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_if_a_probe_is_in_the_goal() {
        let position = simulate_probe_arc(Velocity { x: 7, y: 2 }, 6);
        let result = check_probe_is_in_goal(&position, (20..30, 10..-5));
        assert_eq!(result, true);
    }
}
