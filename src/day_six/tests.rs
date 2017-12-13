mod part_one {
    use day_six::*;

    #[test]
    fn it_redistributes_the_blocks() {
        let input = &[0, 2, 7, 0];

        let mut r = Reallocator::new(input);
        assert_eq!(r.next(), Some(vec![2, 4, 1, 2]));
    }

    #[test]
    fn it_finds_the_infinite_loop() {
        let input = &[0, 2, 7, 0];
        assert_eq!(detect_infinite_loop(input), 5);
    }

    #[test]
    fn the_answer_is() {
        let input = &[2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14];
        assert_eq!(detect_infinite_loop(input), 3156);
    }
}

mod part_two {
    use day_six::*;

    #[test]
    fn it_finds_the_loop_distance() {
        let input = &[0, 2, 7, 0];
        assert_eq!(find_loop_length(input), 4);
    }

    #[test]
    fn the_answer_is() {
        let input = &[2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14];
        assert_eq!(find_loop_length(input), 1610);
    }
}
