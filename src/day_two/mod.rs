#[cfg(test)]
mod tests;
pub mod input;

pub fn calculate_checksum(spreadsheet: &[&[u32]]) -> u32 {
    spreadsheet.iter().map(|x| row_diff(x)).sum()
}

pub fn row_diff(row: &[u32]) -> u32 {
    row.iter().max().expect("no max!") - row.iter().min().expect("no min!")
}

pub fn part_two_checksum(s: &[&[u32]]) -> u32 {
    s.iter().map(|x| row_division(x)).sum()
}

pub fn row_division(row: &[u32]) -> u32 {
    for item in row {
        for other_item in row {
            if item % other_item == 0 && item != other_item {
                return item / other_item;
            }
        }
    }

    0
}
