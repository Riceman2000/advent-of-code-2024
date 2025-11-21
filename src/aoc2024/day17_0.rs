use std::mem::transmute;

use atoi::atoi;

#[derive(aoc_macros::AocDay)]
#[output_type("String")]
#[expected_short(None)]
#[expected_long(Some("2,1,0,1,7,2,5,0,3".to_string()))]
pub struct Day;

#[must_use]
#[allow(clippy::cast_lossless)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::cast_possible_truncation)]
pub fn day(input: &[u8]) -> String {
    let mut lines = input.trim_ascii().split(|b| *b == b'\n');
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
    let mut out = Vec::new();
    while ip < instructions.len() {
        let opcode: Opcode = unsafe { transmute(instructions[ip]) }; // Teehee
        let operand = instructions[ip + 1] as u64;

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
            Opcode::Out => out.push(combo_op!(operand) % 8),
            Opcode::Bdv => b = a / 2u64.pow(combo_op!(operand) as u32),
            Opcode::Cdv => c = a / 2u64.pow(combo_op!(operand) as u32),
        }
        ip += 2;
    }

    let out = out
        .iter()
        .fold(String::new(), |acc, n| acc + "," + &n.to_string());
    out[1..].to_string()
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
