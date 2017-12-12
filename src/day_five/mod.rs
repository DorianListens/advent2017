#[cfg(test)]
mod tests;
mod input;

pub fn count_jumps_to_exit(list: Vec<i32>) -> u32 {
    let mut p = Processor::new(list);
    let mut n = 0;
    while let Some(_) = p.process() {
        n += 1;
    }
    n
}

pub fn count_part_two(list: Vec<i32>) -> u32 {
    let mut p = Processor::new(list);
    let mut n = 0;
    while let Some(_) = p.process_part_two() {
        n += 1;
    }
    n
}

struct Processor {
    list: Vec<i32>,
    index: usize,
}

impl Processor {
    fn new(list: Vec<i32>) -> Processor {
        Processor {
            list,
            index: 0,
        }
    }

    fn process(&mut self) -> Option<usize> {
        self.p(Processor::increment)
    }

    fn process_part_two(&mut self) -> Option<usize> {
        self.p(Processor::increment_or_decrement)
    }

    fn p<T>(&mut self, f: T) -> Option<usize> where T: Fn(&mut Processor, i32) {
        if self.list.len() > self.index {
            let steps_to_take = self.list[self.index];
            f(self, steps_to_take);
            if steps_to_take < 0 {
                self.index -= steps_to_take.abs() as usize;
            } else {
                self.index += steps_to_take as usize;
            }
            return Some(self.index);
        }
        None
    }

    fn increment(&mut self, steps_to_take: i32) {
        self.list[self.index] += 1;
    }

    fn increment_or_decrement(&mut self, steps_to_take: i32) {
        if steps_to_take >= 3 {
            self.list[self.index] -= 1;
        } else {
            self.list[self.index] += 1;
        }
    }
}

