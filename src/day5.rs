use std::io::Write;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list0, sequence::separated_pair,
};

use crate::utils;

#[derive(Debug)]
struct Rules {
    pub rules: Vec<(u8, u8)>,
}
impl Rules {
    fn check(&self, vals: [u8; 100]) -> bool {
        for rule in &self.rules {
            let first = vals[rule.0 as usize];
            let second = vals[rule.1 as usize];
            if first != u8::MAX && second != u8::MAX && second < first {
                return false;
            }
        }
        true
    }
}
fn parse_rules(input: &str) -> (Rules, &str) {
    let mut rules = separated_list0(
        tag::<_, _, ()>("\n"),
        separated_pair(complete::u8, tag("|"), complete::u8),
    );
    let (tail, rules) = rules(input).unwrap();
    (Rules { rules }, &tail[2..])
}
fn parse_vals(mut input: &[u8]) -> Vec<([u8; 100], usize)> {
    let mut answer = Vec::new();

    let mut i = 0;
    let mut curr = [u8::MAX; 100];
    let mut len = 0;
    loop {
        let (a, tail) = utils::parse_2digit(input);
        len += 1;
        curr[a as usize] = i;
        i += 1;
        if tail.len() <= 1 {
            answer.push((curr, len));
            return answer;
        }
        if tail[0] == b'\n' {
            answer.push((curr, len));
            i = 0;
            len = 0;
            curr = [u8::MAX; 100];
        }
        input = &tail[1..];
    }
}
pub fn part_1(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
    let (rules, tail) = parse_rules(input);
    let vals = parse_vals(tail.as_bytes());
    // dbg!(rules.check(vals[0]));
    let mut sum = 0;
    for (vals, len) in vals {
        if rules.check(vals) {
            for (i, val) in vals.iter().enumerate() {
                if *val as usize == (len - 1) / 2 {
                    sum += i;
                    break;
                }
            }
        }
    }

    writeln!(output, "{sum}")?;
    Ok(())
}
pub fn part_2(input: &str, output: &mut impl Write) -> anyhow::Result<()> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
    ";

    #[test]
    fn day_5_1() {
        let input = INPUT.trim();
        let answer = 143.to_string();
        let mut my_answer = Vec::new();
        part_1(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
    #[test]
    fn day_5_2() {
        let input = INPUT.trim();
        let answer = 9.to_string();
        let mut my_answer = Vec::new();
        part_2(input, &mut my_answer).unwrap();
        assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
    }
}