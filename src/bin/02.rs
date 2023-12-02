use std::{ops::Add, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BagOfBall {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl BagOfBall {
    pub fn is_valid(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }

    pub fn max(&self, other: Self) -> Self {
        Self {
            red: {
                if self.red > other.red {
                    self.red
                } else {
                    other.red
                }
            },
            green: {
                if self.green > other.green {
                    self.green
                } else {
                    other.green
                }
            },
            blue: {
                if self.blue > other.blue {
                    self.blue
                } else {
                    other.blue
                }
            },
        }
    }

    pub fn power(&self) -> u32 {
        return self.red * self.blue * self.green;
    }
}

impl Add for BagOfBall {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

pub fn count_elements(input: &str) -> BagOfBall {
    // This method is a bit overengineered, since I thougt an valid choice would be
    // 3 green, 2 green, 1 red and this should lead to 5 green, 1 red.
    // Turns out this is not the case, but since the method still works, I keep it here.
    let mut bag_of_ball = BagOfBall {
        red: 0,
        green: 0,
        blue: 0,
    };

    for element in input.split(", ") {
        let mut it = element.split(' ');
        let amount: u32 = FromStr::from_str(it.next().unwrap()).unwrap();
        let color = it.next().unwrap();

        match color {
            "red" => bag_of_ball.red += amount,
            "green" => bag_of_ball.green += amount,
            "blue" => bag_of_ball.blue += amount,
            _ => (),
        }
    }
    return bag_of_ball;
}

pub fn evaluate_game(input: &str) -> Option<u32> {
    // Evaluates if a game is valid. Returns the game number if the game is valid.
    let mut game_information = input.split(": ");
    let game: u32 =
        FromStr::from_str(game_information.next().unwrap().split(" ").nth(1).unwrap()).unwrap();
    let elements = game_information.next().unwrap().split("; ");

    let mut is_valid = true;

    for element in elements {
        let balls_in_drawing = count_elements(element);
        if !balls_in_drawing.is_valid() {
            is_valid = false;
        }
    }
    return is_valid.then_some(game); //current_bag_of_balls.is_valid().then_some(game);
}

pub fn get_minimal_balls(input: &str) -> u32 {
    // Returns the minium required balls to realize that game. Returns the power of the three minimal colors.
    let mut game_information = input.split(": ");
    let elements = game_information.nth(1).unwrap().split("; ");

    let mut minimal_ball: BagOfBall = BagOfBall {
        red: 0,
        green: 0,
        blue: 0,
    };

    for element in elements {
        let balls_in_drawing = count_elements(element);
        minimal_ball = minimal_ball.max(balls_in_drawing);
    }
    return minimal_ball.power();
}

pub fn part_one(input: &str) -> Option<u32> {
    return input.lines().filter_map(evaluate_game).reduce(|a, b| a + b);
}

pub fn part_two(input: &str) -> Option<u32> {
    return input.lines().map(get_minimal_balls).reduce(|a, b| a + b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
