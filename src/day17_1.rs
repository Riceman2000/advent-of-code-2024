use std::mem::transmute;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day17.txt");

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day() -> u64 {
    let mut lines = INPUT.trim_ascii().split(|b| *b == b'\n');
    let mut a = 1;
    let b = 0;
    let c = 0;
    let prog: Vec<_> = lines
        .nth(4)
        .unwrap()
        .split(|b| *b == b',')
        .map(|b| b.last().unwrap() - b'0')
        .collect();

    let mut out = Vec::new();
    loop {
        out.clear();
        output_from_ic(a, b, c, &prog, &mut out);

        if prog.ends_with(&out) {
            if prog.len() == out.len() {
                break a;
            }
            a <<= 3;
        } else {
            // No more solutions
            while a % 8 == 7 {
                a >>= 3;
            }
            a += 1;
        }
    }
}

#[inline]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_possible_truncation)]
fn output_from_ic(mut a: u64, mut b: u64, mut c: u64, prog: &[u8], out: &mut Vec<u8>) {
    macro_rules! combo_op {
        ($operand:expr) => {
            match $operand {
                0..=3 => $operand,
                4 => a,
                5 => b,
                6 => c,
                o => unreachable!("Got invalid operand '{o}'"),
            }
        };
    }
    let mut ip = 0;
    while ip < prog.len() {
        let opcode: Opcode = unsafe { transmute(prog[ip]) }; // Teehee
        let operand = prog[ip + 1] as u64;

        match opcode {
            Opcode::Adv => a /= 2u64.pow(combo_op!(operand) as u32),
            Opcode::Bxl => b ^= operand,
            Opcode::Bst => b = combo_op!(operand) % 8,
            Opcode::Jnz => {
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            Opcode::Bxc => b ^= c,
            Opcode::Out => out.push((combo_op!(operand) % 8) as u8),
            Opcode::Bdv => b = a / 2u64.pow(combo_op!(operand) as u32),
            Opcode::Cdv => c = a / 2u64.pow(combo_op!(operand) as u32),
        }
        ip += 2;
    }
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

/// Used to allow for the verfication of results at runtime without a panic
#[must_use]
pub fn verify_day(print_output: bool) -> bool {
    // Correct value can be put here once it is known
    let expected = 267_265_166_222_235;

    let actual = day();
    if actual == expected {
        return true;
    }

    if print_output {
        // To help handle unsigned subtraction
        let sign = if actual > expected { '+' } else { '-' };
        eprintln!(
            "Got {actual} expected {expected}, diff {sign}{}",
            expected.abs_diff(actual)
        );
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that is automatically run by `cargo test`
    #[test]
    fn test_day() {
        assert!(verify_day(true));
    }
}
