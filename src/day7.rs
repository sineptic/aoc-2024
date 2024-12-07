use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list0,
    sequence::separated_pair,
};

fn parse(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> {
    let line_parser = separated_pair(u64::<_, ()>, tag(": "), separated_list0(tag(" "), u64));
    let mut parser = separated_list0(tag("\n"), line_parser);
    let (tail, result) = parser(input).unwrap();
    assert!(tail.trim().is_empty());
    result.into_iter()
}

struct Equation<'a> {
    needed: u64,
    current: u64,
    numbers: &'a [u64],
}
fn could_be_true_inner(
    Equation {
        needed,
        current,
        numbers,
    }: Equation,
) -> bool {
    if numbers.is_empty() {
        return needed == current;
    }
    if current > needed {
        return false;
    }
    could_be_true_inner(Equation {
        needed,
        current: current + numbers[0],
        numbers: &numbers[1..],
    }) || could_be_true_inner(Equation {
        needed,
        current: current * numbers[0],
        numbers: &numbers[1..],
    })
}
fn could_be_true(needed: u64, numbers: &[u64]) -> bool {
    could_be_true_inner(Equation {
        needed,
        current: numbers[0],
        numbers: &numbers[1..],
    })
}
pub fn part_1(input: &str, output: &mut impl std::io::Write) -> anyhow::Result<()> {
    let answer = parse(input)
        .map(|(needed, numbers)| {
            if could_be_true(needed, &numbers) {
                needed
            } else {
                0
            }
        })
        .sum::<u64>();

    writeln!(output, "{answer}")?;
    Ok(())
}
fn could_be_true2_inner(
    Equation {
        needed,
        current,
        numbers,
    }: Equation,
) -> bool {
    if numbers.is_empty() {
        return needed == current;
    }
    if current > needed {
        return false;
    }
    fn concat(a: u64, b: u64) -> u64 {
        let log10 = (b as f64).log10() as u32;
        a * 10_u64.pow(log10 + 1) + b
    }
    could_be_true2_inner(Equation {
        needed,
        current: current + numbers[0],
        numbers: &numbers[1..],
    }) || could_be_true2_inner(Equation {
        needed,
        current: current * numbers[0],
        numbers: &numbers[1..],
    }) || could_be_true2_inner(Equation {
        needed,
        current: concat(current, numbers[0]),
        numbers: &numbers[1..],
    })
}
fn could_be_true2(needed: u64, numbers: &[u64]) -> bool {
    could_be_true2_inner(Equation {
        needed,
        current: numbers[0],
        numbers: &numbers[1..],
    })
}
pub fn part_2(input: &str, output: &mut impl std::io::Write) -> anyhow::Result<()> {
    let answer = parse(input)
        .map(|(needed, numbers)| {
            if could_be_true2(needed, &numbers) {
                needed
            } else {
                0
            }
        })
        .sum::<u64>();

    writeln!(output, "{answer}")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn day_7_1() {
        let input = INPUT.trim();
        let answer = 3749.to_string();
        let mut my_answer = Vec::new();
        part_1(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
    #[test]
    fn day_7_2() {
        let input = INPUT.trim();
        let answer = 11387.to_string();
        let mut my_answer = Vec::new();
        part_2(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
}
