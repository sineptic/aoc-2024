macro_rules! test {
    ($day:ident) => {
        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn $day() {
                let input = include_str!("../example.txt");
                let answer = include_str!("../answer.txt");
                let mut my_answer = Vec::new();
                solve(input, &mut my_answer).unwrap();
                assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
            }
        }
    };
}

pub mod day1 {
    use std::io::Write;

    use anyhow::Result;

    pub fn solve(input: &str, output: &mut impl Write) -> Result<()> {
        Ok(())
    }

    test!(day1);
}
