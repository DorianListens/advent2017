#[cfg(test)]
mod tests;

use std::iter::Iterator;
use std::collections::{HashMap, HashSet};

pub fn detect_infinite_loop(input: &[u32]) -> u32 {
    let r = Reallocator::new(input);
    let mut previous_patterns = HashSet::<Vec<u32>>::new();
    let mut count = 0;
    for pattern in r {
        count += 1;
        if previous_patterns.contains(&pattern) {
            break;
        }
        previous_patterns.insert(pattern);
    }

    count
}

pub fn find_loop_length(input: &[u32]) -> u32 {
    let r = Reallocator::new(input);
    let mut previous_patterns = HashMap::<Vec<u32>, usize>::new();
    for (index, pattern) in r.enumerate() {
        if let Some(first_index) = previous_patterns.get(&pattern) {
            return (index - *first_index) as u32;
        }
        previous_patterns.insert(pattern, index);
    }

    0
}

pub struct Reallocator {
    memory: Vec<u32>,
}

impl Reallocator {
    fn new(memory: &[u32]) -> Reallocator {
        Reallocator {
            memory: memory.to_vec(),
        }
    }

    fn reallocate(&mut self) -> Vec<u32> {
        let mut new_memory = self.memory.clone();
        let max = self.memory.iter().max().expect("no max value");
        let mut index = self.memory.iter().position(|x| x == max).expect("no index");
        let length = self.memory.len();

        new_memory[index] = 0;
        for _ in 0..*max {
            index = match index {
                x if x >= length - 1 => 0,
                x => x + 1,
            };

            new_memory[index] += 1;
        }

        new_memory
    }
}

impl Iterator for Reallocator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        let next = self.reallocate();
        self.memory = next;
        Some(self.memory.clone())
    }
}
