mod part_one {
    use day_five::*;

    #[test]
    fn a_list_of_zero_takes_two_steps() {
        let list = vec![0];
        assert_eq!(count_jumps_to_exit(list), 2);
    }

    #[test]
    fn a_list_of_one_takes_one_step() {
        let list = vec![1];
        assert_eq!(count_jumps_to_exit(list), 1);
    }

    #[test]
    fn the_example_takes_five_steps() {
        let list = vec![0, 3, 0, 1, -3];
        assert_eq!(count_jumps_to_exit(list), 5);
    }

    #[test]
    fn the_answer_is() {
        let list = input::INPUT.to_vec();
        assert_eq!(count_jumps_to_exit(list), 351282);
    }
}

mod part_two {
    use day_five::*;

    #[test]
    fn the_example_takes_five_steps() {
        let list = vec![0, 3, 0, 1, -3];
        assert_eq!(count_part_two(list), 10);
    }

    #[test]
    #[ignore]
    fn the_answer_is() {
        let list = input::INPUT.to_vec();
        assert_eq!(count_part_two(list), 24568703);
    }
}
