#[cfg(test)]
mod tests;
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
