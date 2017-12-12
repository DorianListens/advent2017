mod part_one {
    use day_two::*;

    #[test]
    fn it_finds_the_difference_between_min_and_max() {
        assert_eq!(row_diff(&[5, 1, 9, 5]), 8);
        assert_eq!(row_diff(&[7, 5, 3]), 4);
        assert_eq!(row_diff(&[2, 4, 6, 8]), 6);
    }

    #[test]
    fn it_calculates_the_checksum() {
        let spreadsheet: &[&[u32]] = &[&[5, 1, 9, 5], &[7, 5, 3], &[2, 4, 6, 8]];

        assert_eq!(calculate_checksum(spreadsheet), 18);
    }

    #[test]
    fn the_answer_is() {
        assert_eq!(calculate_checksum(input::PUZZLE_INPUT), 34925);
    }
}

mod part_two {
    use day_two::*;

    #[test]
    fn it_finds_the_even_division() {
        assert_eq!(row_division(&[5, 9, 2, 8]), 4);
        assert_eq!(row_division(&[9, 4, 7, 3]), 3);
        assert_eq!(row_division(&[3, 8, 6, 5]), 2);
    }

    #[test]
    fn the_answer_is() {
        assert_eq!(part_two_checksum(input::PUZZLE_INPUT), 221);
    }
}
