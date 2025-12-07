use std::{io::BufRead, iter};

pub fn a(input: impl BufRead) -> u64 {
    let mut dial = 50i16;
    let mut zero = 0;

    for line in input.lines() {
        let line = line.unwrap();
        let (direction, amount) = line.split_at(1);
        let rotate = amount.parse::<i16>().unwrap()
            * match direction {
                "L" => -1,
                "R" => 1,
                _ => panic!(),
            };
        dial += rotate;
        while dial < 0 {
            dial += 100;
        }
        while dial >= 100 {
            dial -= 100;
        }
        if dial == 0 {
            zero += 1;
        }
    }
    zero
}

pub fn b(input: impl BufRead) -> u64 {
    let mut dial = 50i16;
    let mut zero = 0;

    for click in input.lines().flat_map(|line| {
        let line = line.unwrap();
        let (direction, amount) = line.split_at(1);
        let amount = amount.parse::<usize>().unwrap();
        match direction {
            "L" => iter::repeat_n(-1, amount),
            "R" => iter::repeat_n(1, amount),
            _ => panic!(),
        }
    }) {
        dial += click;
        if dial < 0 {
            dial += 100;
        } else if dial >= 100 {
            dial -= 100;
        }
        if dial == 0 {
            zero += 1;
        }
    }
    zero
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_str;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_a() {
        let result = a(read_str(EXAMPLE));
        assert_eq!(3, result);
    }

    #[test]
    fn test_b() {
        let result = b(read_str(EXAMPLE));
        assert_eq!(6, result);
    }
}
