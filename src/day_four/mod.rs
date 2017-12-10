pub mod input;

pub fn validate(passphrase: &str) -> bool {
    let words = split_into_words(passphrase);
    !contains_duplicates(words)
}

fn split_into_words(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn contains_duplicates(mut words: Vec<&str>) -> bool {
    words.sort();
    words.windows(2).any(|x| x[0] == x[1])
}

#[cfg(test)]
mod tests {
    mod part_one {
        use day_four::*;
        #[test]
        fn first_example_is_valid() {
            assert_eq!(validate(&"aa bb cc dd ee"), true);
        }

        #[test]
        fn second_example_is_not_valid() {
            assert_eq!(validate(&"aa bb cc dd aa"), false);
        }

        #[test]
        fn third_example_is_valid() {
            assert_eq!(validate(&"aa bb cc dd aaa"), true);
        }

        #[test]
        fn the_answer_is() {
            let answer = input::PUZZLE_INPUT.lines().filter(|x| validate(x)).collect::<Vec<_>>().len();
            assert_eq!(answer, 383);
        }
    }
}
