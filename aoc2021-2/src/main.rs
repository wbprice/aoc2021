use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("couldn't read the file")
        .lines()
        .map(|line| line.parse().expect("couldn't parse line"))
        .collect();

    let start = (0, 0);
    let (x, y) = execute_movements(input, start);
    dbg!(x, y);
    dbg!(x as u32 * y as u32);
}

fn parse_movement(string: String) -> (String, u16) {
    let mut iter = string.split_whitespace();
    let dir = iter.next().unwrap();
    let dist = iter.next().unwrap();
    let direction = dir.to_string();
    let distance = dist
        .parse::<u16>()
        .expect("couldn't parse string as integer");

    (direction, distance)
}

fn execute_movement(movement: String, position: (u16, u16)) -> (u16, u16) {
    let (direction, distance) = parse_movement(movement);
    match direction.as_str() {
        "forward" => (position.0 + distance, position.1),
        "up" => (position.0, position.1 - distance),
        "down" => (position.0, position.1 + distance),
        _ => position,
    }
}

fn execute_movements(movements: Vec<String>, position: (u16, u16)) -> (u16, u16) {
    let mut pos = position.clone();
    for movement in movements {
        pos = execute_movement(movement, pos);
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_works() {
        let input = "forward 5".to_string();
        let (direction, distance) = parse_movement(input);
        assert_eq!(direction, "forward");
        assert_eq!(distance, 5);
    }

    #[test]
    fn execute_movement_works() {
        let movement = "forward 5".to_string();
        let start = (0, 0);
        let (x, y) = execute_movement(movement, start);
        assert_eq!(x, 5);
        assert_eq!(y, 0);
    }

    #[test]
    fn execute_movements_works() {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        let start = (0, 0);
        let (x, y) = execute_movements(input, start);
        assert_eq!(x, 15);
        assert_eq!(y, 10);
        assert_eq!(x * y, 150)
    }
}
