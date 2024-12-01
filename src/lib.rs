#![feature(exact_size_is_empty)]

pub mod day1 {
    use std::io::Write;

    use anyhow::Result;
    use itertools::Itertools;

    pub fn part_1(input: &str, output: &mut impl Write) -> Result<()> {
        let (first, second): (Vec<String>, Vec<String>) = input
            .lines()
            .map(|line| {
                let mut numbers = line
                    .split_whitespace()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>();
                assert_eq!(numbers.len(), 2);
                let second = numbers.pop().unwrap();
                let first = numbers.pop().unwrap();

                (first, second)
            })
            .unzip();
        let first = first
            .into_iter()
            .map(|x| x.parse::<u64>().unwrap())
            .sorted();
        let second = second
            .into_iter()
            .map(|x| x.parse::<u64>().unwrap())
            .sorted();

        let answer = first.zip(second).map(|(a, b)| a.abs_diff(b)).sum::<u64>();

        writeln!(output, "{answer}")?;

        Ok(())
    }
    pub fn part_2(input: &str, output: &mut impl Write) -> Result<()> {
        let (first, second): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let mut numbers = line
                    .split_whitespace()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>();
                assert_eq!(numbers.len(), 2);
                let second = numbers.pop().unwrap().parse::<usize>().unwrap();
                let first = numbers.pop().unwrap().parse::<usize>().unwrap();

                (first, second)
            })
            .unzip();

        let mut first = first.into_iter().sorted();
        let mut second = second.into_iter().sorted();
        let mut answer = 0;
        loop {
            if first.is_empty() || second.is_empty() {
                break;
            }
            let current_number = first.next().unwrap();
            let first_count = first.take_while_ref(|x| *x == current_number).count() + 1;

            second
                .take_while_ref(|x| *x < current_number)
                .for_each(|_| {});
            let second_count = second.take_while_ref(|x| *x == current_number).count();

            answer += current_number * first_count * second_count;
        }

        writeln!(output, "{answer}")?;

        Ok(())
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn day_1_1() {
            let input = "
3   4
4   3
2   5
1   3
3   9
3   3
"
            .trim();
            let answer = "
11
"
            .trim();
            let mut my_answer = Vec::new();
            part_1(input, &mut my_answer).unwrap();
            assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
        }
        #[test]
        fn day_1_2() {
            let input = "
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "
            .trim();
            let answer = "
    31
    "
            .trim();
            let mut my_answer = Vec::new();
            part_2(input, &mut my_answer).unwrap();
            assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
        }
    }
}
