use std::iter::Iterator;

fn steps_away(n: u32) -> u32 {
    let mut spiral = Spiral::new();
    let tile = spiral.find(|x| x.number == n).expect("infinite sequence");
    manhattan_distance(tile.point)
}

fn manhattan_distance(point: Point) -> u32 {
    (point.x.abs() + point.y.abs()) as u32
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    number: u32,
    point: Point,
}

impl Tile {
    fn one() -> Tile {
        Tile {
            number: 1,
            point: Point { x: 0, y: 0 },
        }
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Spiral {
    current: Tile,
    next: Tile,
    dir: Direction,
    x_steps: u32,
    y_steps: u32,
    steps_taken: u32,
}

use self::Direction::*;

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            current: Tile::one(),
            next: Tile::one(),
            dir: Direction::Down,
            x_steps: 1,
            y_steps: 0,
            steps_taken: 0,
        }
    }

    pub fn next_tile(&mut self) -> Tile {
        let dist_to_compare = match self.dir {
            Left | Right => self.x_steps,
            Up | Down => self.y_steps,
        };

        if self.steps_taken == dist_to_compare {
            self.increment_steps();
            self.dir = self.next_dir();
            self.steps_taken = 0;

            self.next_step()
        } else {
            self.next_step()
        }
    }

    fn increment_steps(&mut self) {
        match self.dir {
            Left | Right => self.x_steps += 1,
            Up | Down => self.y_steps += 1,
        };
    }

    fn next_step(&mut self) -> Tile {
       let next_point = match self.dir {
           Up => Point { x: self.current.point.x, y: self.current.point.y + 1 },
           Down => Point { x: self.current.point.x, y: self.current.point.y - 1 },
           Left => Point { x: self.current.point.x - 1, y: self.current.point.y },
           Right => Point { x: self.current.point.x + 1, y: self.current.point.y },
       };

       self.steps_taken += 1;
       Tile {
           number: self.current.number + 1,
           point: next_point,
       }
    }

    fn next_dir(&self) -> Direction {
        match self.dir {
            Down => Right,
            Right => Up,
            Up => Left,
            Left => Down,
        }
    }
}

impl Iterator for Spiral {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        self.current = self.next;
        self.next = self.next_tile();
        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    mod part_one {
        use day_three::*;

        #[test]
        fn the_first_number_is_at_zero_zero() {
            let mut spiral = Spiral::new();

            let first = spiral.next().expect("Sprial is infinite");

            assert_eq!(first.point, Point { x: 0 , y: 0 });
            assert_eq!(first.number, 1);
        }

        #[test]
        fn the_second_number_is_at_one_zero() {
            let mut spiral = Spiral::new();

            spiral.next();
            let second = spiral.next().expect("Sprial is infinite");

            assert_eq!(second.point, Point { x: 1 , y: 0 });
        }

        #[test]
        fn the_third_number_is_at_one_one() {
            let mut spiral = Spiral::new();

            spiral.next();
            spiral.next();
            let third = spiral.next().expect("Sprial is infinite");

            assert_eq!(third.point, Point { x: 1 , y: 1 });
        }

        #[test]
        fn the_fourth_number_is_at_zero_one() {
            let mut spiral = Spiral::new();

            spiral.next();
            spiral.next();
            spiral.next();
            let fourth = spiral.next().expect("Sprial is infinite");

            assert_eq!(fourth.point, Point { x: 0 , y: 1 });
        }

        #[test]
        fn twenty_five_is_at_two_minus_two() {
            let mut spiral = Spiral::new();

            for _ in 0..24 {
                spiral.next();
            }

            let tile = spiral.next().expect("Sprial is infinite");

            assert_eq!(tile.point, Point { x: 2, y: -2 });
            assert_eq!(tile.number, 25);
        }

        #[test]
        fn manhattan_of_origin_is_zero() {
            let origin = Point { x: 0, y: 0 };
            assert_eq!(manhattan_distance(origin), 0);
        }

        #[test]
        fn manhattan_of_one_one_is_two() {
            let origin = Point { x: 1, y: 1 };
            assert_eq!(manhattan_distance(origin), 2);
        }

        #[test]
        fn square_one_is_zero_step() {
            assert_eq!(steps_away(1), 0);
        }

        #[test]
        fn square_twelve_is_three_steps() {
            assert_eq!(steps_away(12), 3);
        }

        #[test]
        fn square_twenty_three_is_two_steps() {
            assert_eq!(steps_away(23), 2);
        }

        #[test]
        fn the_answer_is() {
            assert_eq!(steps_away(277678), 475);
        }
    }
}
