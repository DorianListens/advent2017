pub mod input;

pub fn rotated_inverse_captcha(input: &str) -> u32 {
    let advanced_by_half = input
        .chars()
        .cycle()
        .skip(input.len() / 2)
        .take(input.len());

    input
        .chars()
        .zip(advanced_by_half)
        .filter(|x| x.0 == x.1)
        .filter_map(|x| x.0.to_digit(10))
        .sum()
}

pub fn inverse_captcha(input: &str) -> u32 {
    let advanced_by_one = input.chars().cycle().skip(1).take(input.len());

    input
        .chars()
        .zip(advanced_by_one)
        .filter(|x| x.0 == x.1)
        .filter_map(|x| x.0.to_digit(10))
        .sum()
}

#[cfg(test)]
mod tests {
    mod part_one {
        use day_one::*;
        #[test]
        fn oneonetwotwo_is_three() {
            assert_eq!(inverse_captcha("1122"), 3);
        }

        #[test]
        fn four_ones_are_four() {
            assert_eq!(inverse_captcha("1111"), 4);
        }

        #[test]
        fn one_two_three_four_is_zero() {
            assert_eq!(inverse_captcha("1234"), 0);
        }

        #[test]
        fn the_last_example_is_nine() {
            assert_eq!(inverse_captcha("91212129"), 9);
        }

        #[test]
        fn the_answer_to_part_one_is() {
            assert_eq!(inverse_captcha(input::PUZZLE_INPUT), 1253);
        }
    }

    mod part_two {
        use day_one::*;
        #[test]
        fn one_two_one_two_is_six() {
            assert_eq!(rotated_inverse_captcha("1212"), 6);
        }

        #[test]
        fn one_two_two_one_is_zero() {
            assert_eq!(rotated_inverse_captcha("1221"), 0);
        }

        #[test]
        fn example_three_is_four() {
            assert_eq!(rotated_inverse_captcha("123425"), 4);
        }

        #[test]
        fn example_four_is_twelve() {
            assert_eq!(rotated_inverse_captcha("123123"), 12);
        }

        #[test]
        fn example_five_is_four() {
            assert_eq!(rotated_inverse_captcha("12131415"), 4);
        }

        #[test]
        fn the_answer_is() {
            assert_eq!(rotated_inverse_captcha(input::PUZZLE_INPUT), 1278);
        }
    }
}
