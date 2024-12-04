fn next_mul(mut input: &str) -> Option<(usize, (i32, i32), &str)> {
    const PREFIX: &str = "mul(";
    let mut offset = 0;

    while let Some((before_mul, after_mul)) = input.split_once(PREFIX) {
        // If there is no non-digit char, then it is EOF
        let a_end = after_mul.find(|c: char| !c.is_ascii_digit())?;
        let (a, after_a) = after_mul.split_at(a_end);

        let Some(after_comma) = after_a.strip_prefix(',') else {
            // If there is no post-digit comma, this mul is invalid through a
            input = after_a;
            offset += before_mul.len() + PREFIX.len() + a.len();
            continue;
        };

        // If there is no non-digit char, then it is EOF
        let b_end = after_comma.find(|c: char| !c.is_ascii_digit())?;
        let (b, after_b) = after_comma.split_at(b_end);

        let Some(after_paren) = after_b.strip_prefix(')') else {
            // If there is no post-digit paren, this mul is invalid though b
            input = after_b;
            offset += before_mul.len() + PREFIX.len() + a.len() + ",".len() + b.len();
            continue;
        };

        return Some((
            offset + before_mul.len(),
            (a.parse().unwrap(), b.parse().unwrap()), // Always valid because of char::is_ascii_digit
            after_paren,
        ))
    }

    None
}

fn next_valid_mul(mut input: &str) -> Option<((i32, i32), &str)> {
    loop {
        // If there is no more `mul()`, we are done
        let (offset, mul, suffix) = next_mul(input)?;

        // If `don't()` is before the mul, skip to next `do()`
        if let Some(dont_idx) = input.find("don't()") && dont_idx < offset {
            let do_idx = input[dont_idx..].find("do()")?;
            input = &input[dont_idx + do_idx..];
            continue;
        }

        return Some((mul, suffix));
    }
}

pub fn main() {
    let input = include_str!("../inputs/input-3.txt");

    let sum = gen {
        let mut input = input;
        while let Some((_, mul, rest)) = next_mul(input) {
            yield mul;
            input = rest;
        }
    }.map(|(a, b)| a * b).sum::<i32>();

    println!("Part 1 | Sum: {}", sum);


    let sum = gen {
        let mut input = input;
        while let Some((mul, rest)) = next_valid_mul(input) {
            yield mul;
            input = rest;
        }
    }.map(|(a, b)| a * b).sum::<i32>();

    println!("Part 2 | Sum: {}", sum);
}
