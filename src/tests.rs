use super::*;
#[test]
fn check_base3() {
    let three = Base::<3>::from(11);
    assert_eq!(
        Base {
            number: 11,
            exponents: vec![(Multiplier(1), Power(2)), (Multiplier(0), Power(1)), (Multiplier(2), Power(0))],
            reduced: false
        },
        three
    );
    println!("{:?}", three);
    let three_into = Into::<u32>::into(three);
    assert_eq!(three_into, 11);
}
#[test]
fn check_base2() {
    let eleven_two = Base::<2>::from(11);
    assert_eq!(
        Base {
            number: 11, // 1011_base2
            exponents: vec![(Multiplier(1), Power(3)), (Multiplier(1), Power(1)), (Multiplier(1), Power(0))],
            reduced: false
        },
        eleven_two
    );
    // println!("{:?}", eleven_two);
    let eleven_into = Into::<u32>::into(eleven_two);
    assert_eq!(eleven_into, 11);
}
// #[ignore = "base 2 infinite loop"]
#[test]
fn check_compute() {
    let eleven_two = Base::<2>::from(11); // loop never terminates
    let eleven_three = Base::<3>::from(11);
    println!("Base 2 -------> {eleven_two:?}");
    println!("Base 3 -------> {eleven_three:?}");
    assert_eq!(eleven_two.compute(), eleven_three.compute());
}
#[test]
fn is_power_of_test() {
    let res = is_power_of(3, 2);
    assert_eq!(res, Some((false, 1)));
    let res = is_power_of(11, 3);
    assert_eq!(res, Some((false, 2)));
    let res = is_power_of(2, 3);
    assert_eq!(res, None);
}