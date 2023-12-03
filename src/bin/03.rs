advent_of_code::solution!(3);

pub fn get_part_number_at_location(input: &str, x: usize, y: usize) -> Option<u32> {
    let line = input.lines().nth(x).unwrap_or("");

    if y >= line.len() {
        return None;
    }

    let linechars: Vec<char> = line.chars().collect();

    // Check the start of the number
    let mut real_y: usize = y;
    loop {
        let tmpchar = linechars.get(real_y).unwrap();
        if tmpchar.is_digit(10) {
            if real_y == 0 {
                break;
            }
            real_y -= 1;
        } else {
            real_y += 1;
            break;
        }
    }

    if real_y > y {
        return None;
    }

    // Get the number
    let mut number = String::from("");
    let mut current_index: usize = real_y;
    while current_index < line.len() {
        let currentchar = linechars.get(current_index).unwrap();
        if currentchar.is_digit(10) {
            number.push(*currentchar);
            current_index += 1;
        } else {
            break;
        }
    }
    return Some(number.parse().unwrap());
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut x = 0;
    let mut partsum = 0;

    for line in lines {
        let characters = line.chars();
        let mut y = 0;
        for character in characters {
            if !(character.is_digit(10) || character == '.') {
                // We found a part
                // Top
                let tl = get_part_number_at_location(input, x - 1, y - 1);
                let t = get_part_number_at_location(input, x - 1, y);
                let tr = get_part_number_at_location(input, x - 1, y + 1);

                if t.is_none() {
                    // TL and TR can be different
                    partsum += tl.unwrap_or(0);
                    partsum += tr.unwrap_or(0);
                } else {
                    partsum += t.unwrap_or(0);
                }

                // Same row
                let sl = get_part_number_at_location(input, x, y - 1);
                let sr = get_part_number_at_location(input, x, y + 1);

                partsum += sl.unwrap_or(0);
                partsum += sr.unwrap_or(0);

                // Bottom
                let bl = get_part_number_at_location(input, x + 1, y - 1);
                let b = get_part_number_at_location(input, x + 1, y);
                let br = get_part_number_at_location(input, x + 1, y + 1);

                if b.is_none() {
                    // BL and BR can be different
                    partsum += bl.unwrap_or(0);
                    partsum += br.unwrap_or(0);
                } else {
                    partsum += b.unwrap_or(0);
                }
            }
            y += 1;
        }

        x += 1;
    }
    return Some(partsum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut x = 0;
    let mut partsum = 0;

    for line in lines {
        let characters = line.chars();
        let mut y = 0;
        for character in characters {
            if character == '*' {
                let mut gear_ratios: Vec<u32> = Vec::new();

                // We found a part
                // Top
                let tl = get_part_number_at_location(input, x - 1, y - 1);
                let t = get_part_number_at_location(input, x - 1, y);
                let tr = get_part_number_at_location(input, x - 1, y + 1);

                if t.is_none() {
                    // TL and TR can be different
                    tl.map(|v| gear_ratios.push(v));
                    tr.map(|v| gear_ratios.push(v));
                } else {
                    t.map(|v| gear_ratios.push(v));
                }

                // Same row
                let sl = get_part_number_at_location(input, x, y - 1);
                let sr = get_part_number_at_location(input, x, y + 1);

                sl.map(|v| gear_ratios.push(v));
                sr.map(|v| gear_ratios.push(v));

                // Bottom
                let bl = get_part_number_at_location(input, x + 1, y - 1);
                let b = get_part_number_at_location(input, x + 1, y);
                let br = get_part_number_at_location(input, x + 1, y + 1);

                if b.is_none() {
                    // BL and BR can be different
                    bl.map(|v| gear_ratios.push(v));
                    br.map(|v| gear_ratios.push(v));
                } else {
                    b.map(|v| gear_ratios.push(v));
                }

                if gear_ratios.len() == 2 {
                    partsum += gear_ratios.get(0).unwrap() * gear_ratios.get(1).unwrap();
                }
            }
            y += 1;
        }

        x += 1;
    }
    return Some(partsum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
