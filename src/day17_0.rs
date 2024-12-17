use std::mem::transmute;

use atoi::atoi;

// Pull this file's contents into the binary as a string literal
const INPUT: &[u8] = include_bytes!("../input/day17.txt");

#[must_use]
#[allow(clippy::cast_lossless)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn day() -> usize {
    let mut lines = INPUT.trim_ascii().split(|b| *b == b'\n');
    let mut a = atoi::<u64>(&lines.next().unwrap()[12..]).unwrap();
    let mut b = atoi::<u64>(&lines.next().unwrap()[12..]).unwrap();
    let mut c = atoi::<u64>(&lines.next().unwrap()[12..]).unwrap();
    let mut ip = 0;
    let instructions: Vec<_> = lines
        .nth(1)
        .unwrap()
        .split(|b| *b == b',')
        .map(|b| b.last().unwrap() - b'0')
        .collect();

    println!("a: {a}, b: {b}, c: {c}, instructions: {instructions:?}");

    macro_rules! combo_op {
        ($operand:expr) => {
            match $operand {
                o if (0..=3).contains(&o) => o,
                4 => a,
                5 => b,
                6 => c,
                o => unreachable!("Got invalid operand '{o}'"),
            }
        };
    }
    let mut out = Vec::new();
    while ip < instructions.len() {
        let opcode: Opcode = unsafe { transmute(instructions[ip]) }; // Teehee
        let operand = instructions[ip + 1] as u64;

        println!("{opcode:?} {operand}, a: {a}, b: {b}, ip: {ip}, out: {out:?}");
        match opcode {
            Opcode::Adv => a /= 2u64.pow(combo_op!(operand) as u32),
            Opcode::Bxl => b ^= operand,
            Opcode::Bst => b = combo_op!(operand) % 8,
            Opcode::Jnz => {
                if a != 0 {
                    ip = operand as usize;
                    // println!("\ta: {a}, b: {b}, ip: {ip}, out: {out:?}");
                    continue;
                }
            }
            Opcode::Bxc => b ^= c,
            Opcode::Out => out.push(combo_op!(operand) % 8),
            Opcode::Bdv => b /= 2u64.pow(combo_op!(operand) as u32),
            Opcode::Cdv => c /= 2u64.pow(combo_op!(operand) as u32),
        }
        println!("\ta: {a}, b: {b}, ip: {ip}, out: {out:?}");
        ip += 2;
    }

    for o in out {
        print!("{o},");
    }
    println!();
    0
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
    let expected = 590;

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
