use std::collections::HashMap;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("Couldn't read input file")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let cavern_map = build_cavern_floor_map(&input);
    let risk = calculate_low_point_risk(&cavern_map);
    dbg!(risk);

    let mut basins = measure_basins(&cavern_map);
    basins.sort_unstable();
    basins.reverse();
    let biggest_basins_product = &basins[0..3].iter().product::<i32>();
    dbg!(biggest_basins_product);
}

fn build_cavern_floor_map(input: &[String]) -> HashMap<(i32, i32), i32> {
    let mut output: HashMap<(i32, i32), i32> = HashMap::new();

    for (y, row) in input.iter().enumerate() {
        for (x, column) in row.chars().into_iter().enumerate() {
            output.insert((x as i32, y as i32), column.to_digit(10).unwrap() as i32);
        }
    }

    output
}

fn find_low_points(map: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32)> {
    map.clone()
        .into_keys()
        .filter(|position| is_low_point(position, map))
        .collect()
}

fn measure_basins(map: &HashMap<(i32, i32), i32>) -> Vec<i32> {
    map.clone()
        .into_keys()
        .filter(|position| is_low_point(position, map))
        .map(|low_point| flood_basin(&low_point, map))
        .map(|basin| basin.into_keys().len() as i32)
        .collect()
}

fn get_neighbors(position: &(i32, i32), map: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32)> {
    // A given point can have as many as 4 or as few as 2 neighbors
    [
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
        (position.0 + 1, position.1),
        (position.0 - 1, position.1),
    ]
    .iter()
    .filter_map(|position| match map.get(position).is_some() {
        true => Some(*position),
        false => None,
    })
    .collect()
}

fn get_basin_neighbors(
    position: &(i32, i32),
    cavern: &HashMap<(i32, i32), i32>,
    basin: &HashMap<(i32, i32), i32>,
) -> Vec<(i32, i32)> {
    let maybe_neighbors = get_neighbors(position, cavern);
    maybe_neighbors
        .into_iter()
        .filter(|&position| {
            let neighbor_value = cavern.get(&position).unwrap();
            let not_mapped = basin.get(&position).is_none();
            let less_than_nine = *neighbor_value < 9;
            less_than_nine && not_mapped
        })
        .collect()
}

fn is_low_point(position: &(i32, i32), map: &HashMap<(i32, i32), i32>) -> bool {
    let height = map
        .get(position)
        .expect("Couldn't get this position, check the map");
    // Get the heights the lowest neighbor
    let lowest_heighbor: i32 = get_neighbors(position, map)
        .iter()
        .filter_map(|position| map.get(position).copied())
        .min()
        .expect("Couldn't find lowest neighbor");

    // Compare this position's height lowest neighbor's height
    height < &lowest_heighbor
}

fn flood_basin(
    position: &(i32, i32),
    cavern: &HashMap<(i32, i32), i32>,
) -> HashMap<(i32, i32), i32> {
    let mut basin: HashMap<(i32, i32), i32> = HashMap::new();
    let mut neighbors = get_basin_neighbors(position, cavern, &basin);
    while !neighbors.is_empty() {
        if let Some(neighbor) = neighbors.pop() {
            if let Some(value) = cavern.get(&neighbor) {
                basin.insert(neighbor, *value);
                let mut new_neighbors = get_basin_neighbors(&neighbor, cavern, &basin);
                neighbors.append(&mut new_neighbors);
            }
        }
    }

    basin
}

fn calculate_low_point_risk(map: &HashMap<(i32, i32), i32>) -> i32 {
    find_low_points(map)
        .iter()
        .map(|position| map.get(position).expect("Couldn't find value of position") + 1)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn it_reads_the_input_into_a_map() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern_floor = build_cavern_floor_map(&input);
        assert_eq!(cavern_floor[&(0, 0)], 2);
        assert_eq!(cavern_floor[&(9, 0)], 0);
        assert_eq!(cavern_floor[&(0, 4)], 9);
        assert_eq!(cavern_floor[&(9, 4)], 8);
    }

    #[test]
    fn it_tests_to_see_if_a_point_is_a_low_point() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern_floor = build_cavern_floor_map(&input);
        assert!(is_low_point(&(9, 0), &cavern_floor));
        assert!(!is_low_point(&(0, 0), &cavern_floor));
    }

    #[test]
    fn it_finds_low_points_in_the_cavern_map() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern_floor = build_cavern_floor_map(&input);
        let low_points = find_low_points(&cavern_floor);
        assert_eq!(low_points.len(), 4);
        assert!(low_points.contains(&(1, 0)));
        assert!(low_points.contains(&(9, 0)));
        assert!(low_points.contains(&(2, 2)));
        assert!(low_points.contains(&(6, 4)));
    }

    #[test]
    fn it_calculates_risk_for_low_points() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern_floor = build_cavern_floor_map(&input);
        let risk = calculate_low_point_risk(&cavern_floor);
        assert_eq!(risk, 15);
    }

    #[test]
    fn it_floods_basin_one() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern = build_cavern_floor_map(&input);
        let basin = flood_basin(&(1, 0), &cavern);
        assert_eq!(basin.into_keys().len(), 3);
    }

    #[test]
    fn it_floods_basin_two() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern = build_cavern_floor_map(&input);
        let basin = flood_basin(&(9, 0), &cavern);
        assert_eq!(basin.into_keys().len(), 9);
    }

    #[test]
    fn it_floods_basin_three() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern = build_cavern_floor_map(&input);
        let basin = flood_basin(&(2, 2), &cavern);
        assert_eq!(basin.into_keys().len(), 14);
    }

    #[test]
    fn it_floods_basin_four() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern = build_cavern_floor_map(&input);
        let basin = flood_basin(&(6, 4), &cavern);
        assert_eq!(basin.into_keys().len(), 9);
    }

    #[test]
    fn it_measures_basins() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let cavern = build_cavern_floor_map(&input);
        let basins = measure_basins(&cavern);
        assert_eq!(basins.len(), 4);
        assert!(basins.contains(&3));
        assert!(basins.contains(&9));
        assert!(basins.contains(&14));
        assert!(basins.contains(&9));
    }
}
