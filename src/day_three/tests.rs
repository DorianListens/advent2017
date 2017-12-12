mod part_one {
    use day_three::*;

    #[test]
    fn the_first_number_is_at_zero_zero() {
        let mut spiral = Spiral::new();

        let first = spiral.next().expect("Spiral is infinite");

        assert_eq!(first.point, Point { x: 0, y: 0 });
        assert_eq!(first.number, 1);
    }

    #[test]
    fn the_second_number_is_at_one_zero() {
        let mut spiral = Spiral::new();

        spiral.next();
        let second = spiral.next().expect("Spiral is infinite");

        assert_eq!(second.point, Point { x: 1, y: 0 });
    }

    #[test]
    fn the_third_number_is_at_one_one() {
        let mut spiral = Spiral::new();

        spiral.next();
        spiral.next();
        let third = spiral.next().expect("Spiral is infinite");

        assert_eq!(third.point, Point { x: 1, y: 1 });
    }

    #[test]
    fn the_fourth_number_is_at_zero_one() {
        let mut spiral = Spiral::new();

        spiral.next();
        spiral.next();
        spiral.next();
        let fourth = spiral.next().expect("Spiral is infinite");

        assert_eq!(fourth.point, Point { x: 0, y: 1 });
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
