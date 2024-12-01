use super::*;

#[test]
fn day1() {
    let input = include_str!("../example.txt");
    let answer = include_str!("../answer.txt");
    let mut my_answer = Vec::new();
    day1::solve(input, &mut my_answer).unwrap();
    assert_eq!(String::from_utf8(my_answer).unwrap().trim(), answer.trim());
}
