#[cfg(test)]
mod tests;
mod input;

pub fn count_jumps_to_exit(list: Vec<i32>) -> u32 {
    let mut p = Processor::new(list);
    let mut n = 0;
    loop {
        if let Some(_) = p.process() {
            n += 1;
            continue;
        }
        break;
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
        if self.list.len() > self.index {
            let steps_to_take = self.list[self.index];
            self.list[self.index] += 1;
            if steps_to_take < 0 {
                self.index -= steps_to_take.abs() as usize;
            } else {
                self.index += steps_to_take as usize;
            }
            return Some(self.index);
        }
        None
    }
}

