use std::iter::Iterator;

pub fn steps_away(n: u32) -> u32 {
    let mut spiral = Spiral::new();
    let tile = spiral.find(|x| x.number == n).expect("infinite sequence");
    manhattan_distance(tile.point)
}

fn manhattan_distance(point: Point) -> u32 {
    (point.x.abs() + point.y.abs()) as u32
}

pub fn first_value_larger_than(n: u32) -> u32 {
    let spiral = Spiral::part_two();
    for tile in spiral {
        if tile.number > n {
            return tile.number;
        }
    }

    0
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
        ]
    }
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

use self::Direction::*;

trait Incrementer {
    fn next_number(&mut self, tile: Tile, next_point: Point) -> u32;
}

struct SimpleIncrementer;

impl Incrementer for SimpleIncrementer {
    fn next_number(&mut self, tile: Tile, _: Point) -> u32 {
        tile.number + 1
    }
}

struct ComplexIncrementor {
    past_values: Vec<Tile>
}

impl ComplexIncrementor {
    fn new() -> ComplexIncrementor {
        ComplexIncrementor {
            past_values: vec![],
        }
    }
}

impl Incrementer for ComplexIncrementor {
    fn next_number(&mut self, tile: Tile, next_point: Point) -> u32 {
        self.past_values.push(tile);

        let mut num = 0;
        let neighbours = next_point.neighbours();
        for v in self.past_values.clone() {
            if neighbours.contains(&v.point) {
                num += v.number;
            }
        }

        num
    }
}

pub struct Spiral {
    current: Tile,
    next: Tile,
    dir: Direction,
    x_steps: u32,
    y_steps: u32,
    steps_taken: u32,
    incrementer: Box<Incrementer>,
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            current: Tile::one(),
            next: Tile::one(),
            dir: Direction::Down,
            x_steps: 1,
            y_steps: 0,
            steps_taken: 0,
            incrementer: Box::new(SimpleIncrementer {}),
        }
    }

    pub fn part_two() -> Spiral {
        Spiral {
            current: Tile::one(),
            next: Tile::one(),
            dir: Direction::Down,
            x_steps: 1,
            y_steps: 0,
            steps_taken: 0,
            incrementer: Box::new(ComplexIncrementor::new()),
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
           number: self.incrementer.next_number(self.current, next_point),
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

            let first = spiral.next().expect("Spiral is infinite");

            assert_eq!(first.point, Point { x: 0 , y: 0 });
            assert_eq!(first.number, 1);
        }

        #[test]
        fn the_second_number_is_at_one_zero() {
            let mut spiral = Spiral::new();

            spiral.next();
            let second = spiral.next().expect("Spiral is infinite");

            assert_eq!(second.point, Point { x: 1 , y: 0 });
        }

        #[test]
        fn the_third_number_is_at_one_one() {
            let mut spiral = Spiral::new();

            spiral.next();
            spiral.next();
            let third = spiral.next().expect("Spiral is infinite");

            assert_eq!(third.point, Point { x: 1 , y: 1 });
        }

        #[test]
        fn the_fourth_number_is_at_zero_one() {
            let mut spiral = Spiral::new();

            spiral.next();
            spiral.next();
            spiral.next();
            let fourth = spiral.next().expect("Spiral is infinite");

            assert_eq!(fourth.point, Point { x: 0 , y: 1 });
        }

        #[test]
        fn twenty_five_is_at_two_minus_two() {
            let mut spiral = Spiral::new();

            for _ in 0..24 {
                spiral.next();
            }

            let tile = spiral.next().expect("Spiral is infinite");

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

    mod part_two {
        use day_three::*;

        #[test]
        fn first_two_squares_are_both_one() {
            let mut spiral = Spiral::part_two();
            assert_eq!(spiral.next().expect("infinite").number, 1);
            assert_eq!(spiral.next().expect("infinite").number, 1);
        }

        #[test]
        fn third_tile_is_two() {
            let mut spiral = Spiral::part_two();
            spiral.next();
            spiral.next();
            assert_eq!(spiral.next().expect("infinite").number, 2);
        }

        #[test]
        fn tile_four_is_four() {
            let mut spiral = Spiral::part_two();
            spiral.next();
            spiral.next();
            spiral.next();
            assert_eq!(spiral.next().expect("infinite").number, 4);
        }

        #[test]
        fn tile_nine_is_twenty_five() {
            let mut spiral = Spiral::part_two();
            for _ in 0..8 {
                spiral.next();
            }

            assert_eq!(spiral.next().expect("infinite").number, 25);
        }

        #[test]
        fn first_value_larger_than_five_is_ten() {
            assert_eq!(first_value_larger_than(5), 10);
        }

        #[test]
        fn first_value_larger_than_eighty_is_122() {
            assert_eq!(first_value_larger_than(80), 122);
        }

        #[test]
        fn the_answer_is() {
            assert_eq!(first_value_larger_than(277678), 279138);
        }
    }
}
