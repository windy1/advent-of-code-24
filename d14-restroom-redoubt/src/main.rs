use std::{fmt, fs};

use aoc24::grid::Grid;

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

struct Robots {
    value: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Robots {
    fn new(value: Vec<Robot>, width: i32, height: i32) -> Self {
        Robots {
            value,
            width,
            height,
        }
    }

    fn simulate(&mut self, seconds: i32) {
        for _ in 0..seconds {
            // println!("{}", self);
            self.simulate_second();
        }
    }

    fn simulate_second(&mut self) {
        for robot in self.value.iter_mut() {
            robot.x = (robot.x + robot.vx) % self.width;
            if robot.x < 0 {
                robot.x += self.width;
            }

            robot.y = (robot.y + robot.vy) % self.height;
            if robot.y < 0 {
                robot.y += self.height;
            }
        }
    }

    fn calculate_safety_factor(&self) -> usize {
        let cx = self.width / 2;
        let cy = self.height / 2;

        let q1 = self
            .value
            .iter()
            .filter(|robot| robot.x < cx && robot.y < cy)
            .count();

        let q2 = self
            .value
            .iter()
            .filter(|robot| robot.x > cx && robot.y < cy)
            .count();

        let q3 = self
            .value
            .iter()
            .filter(|robot| robot.x < cx && robot.y > cy)
            .count();

        let q4 = self
            .value
            .iter()
            .filter(|robot| robot.x > cx && robot.y > cy)
            .count();

        q1 * q2 * q3 * q4
    }
}

impl fmt::Display for Robots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = Grid::with_default_char(self.width as usize, self.height as usize, '.');

        for robot in self.value.iter() {
            let curr = grid.get(robot.x as usize, robot.y as usize).to_digit(10);

            if let Some(digit) = curr {
                grid.set(
                    robot.x as usize,
                    robot.y as usize,
                    (digit + 1).to_string().chars().next().unwrap(),
                );
            } else {
                grid.set(robot.x as usize, robot.y as usize, '1');
            }
        }

        write!(f, "{}", grid)
    }
}

impl From<&str> for Robot {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let coords = parse_int_pair(&parts[0][2..]);
        let velocity = parse_int_pair(&parts[1][2..]);

        Robot {
            x: coords[0],
            y: coords[1],
            vx: velocity[0],
            vy: velocity[1],
        }
    }
}

fn parse_int_pair(s: &str) -> Vec<i32> {
    s.split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn main() {
    let file_path = "./d14-restroom-redoubt/input.txt";
    let contents = fs::read_to_string(file_path).unwrap().trim().to_string();

    let seconds = 100;
    let width = 101;
    let height = 103;

    let mut robots = Robots::new(
        contents.lines().map(Robot::from).collect::<Vec<Robot>>(),
        width,
        height,
    );

    robots.simulate(seconds);

    // println!("{}", robots);

    let result = robots.calculate_safety_factor();

    println!("Solution (Part 1): {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_from_str() {
        let input = "p=0,4 v=3,-3";
        let robot = Robot::from(input);
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 4);
        assert_eq!(robot.vx, 3);
        assert_eq!(robot.vy, -3);
    }
}
