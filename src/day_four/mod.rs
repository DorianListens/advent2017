pub mod input;

pub fn part_one(passphrase: &str) -> bool {
    let words = split_into_words(passphrase);
    !contains_duplicates(&words)
}

pub fn part_two(passphrase: &str) -> bool {
    let words = split_into_words(passphrase);
    !contains_duplicates(&words) && !contains_anagrams(words)
}

fn split_into_words(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn contains_duplicates(words: &Vec<&str>) -> bool {
    let mut clone = words.clone();
    clone.sort();
    clone.windows(2).any(|x| x[0] == x[1])
}

fn contains_anagrams(words: Vec<&str>) -> bool {
    let copy = words.clone();
    for word in words {
        for other_word in &copy {
            if &word == other_word {
                continue;
            }

            if word.len() == other_word.len() {
                let mut first = word.clone().chars().collect::<Vec<_>>();
                first.sort();
                let mut second = other_word.clone().chars().collect::<Vec<_>>();
                second.sort();
                if first == second {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    mod part_one {
        use day_four::*;
        #[test]
        fn first_example_is_valid() {
            assert_eq!(part_one(&"aa bb cc dd ee"), true);
        }

        #[test]
        fn second_example_is_not_valid() {
            assert_eq!(part_one(&"aa bb cc dd aa"), false);
        }

        #[test]
        fn third_example_is_valid() {
            assert_eq!(part_one(&"aa bb cc dd aaa"), true);
        }

        #[test]
        fn the_answer_is() {
            let answer = input::PUZZLE_INPUT.lines().filter(|x| part_one(x)).collect::<Vec<_>>().len();
            assert_eq!(answer, 383);
        }
    }

    mod part_two {
        use day_four::*;

        #[test]
        fn first_example_is_valid() {
            assert_eq!(part_two(&"abcde fghij"), true);
        }

        #[test]
        fn second_example_is_not_valid() {
            assert_eq!(part_two(&"abcde xyz ecdab"), false);
        }

        #[test]
        fn third_example_is_valid() {
            assert_eq!(part_two(&"a ab abc abd abf abj"), true);
        }

        #[test]
        fn the_answer_is() {
            let answer = input::PUZZLE_INPUT.lines().filter(|x| part_two(x)).collect::<Vec<_>>().len();
            assert_eq!(answer, 265);
        }
    }
}
