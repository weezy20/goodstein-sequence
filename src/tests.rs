use super::*;
#[test]
fn check_display_impl() {
    use radix_fmt::radix;
    const K: u32 = 8;
    for i in 1..1000_000_999_u32 {
        let r = radix(i, K as u8);
        let b = Base::<K>::from(i);
        println!("Radix: {r} Base<K>: {b:?}");
        assert_eq!(r.to_string(), b.to_string());
    }
}
#[test]
fn check_octal() {
    // let seven = Base::<8>::from(7);
    // Since 7 cannot be further reduced in Base-8
    // println!("{seven:?}");
    // assert_eq!(seven.reduced, true);
    let eighteen = Base::<8>::from(18);
    assert_eq!(
        Base {
            number: 18,
            exponents: vec![
                (Multiplier(2), Power(1)),
                // (Multiplier(0), Power(1)), // This should not be here
                (Multiplier(2), Power(0))
            ],
            reduced: false
        },
        eighteen
    );
    let _69 = Base::<8>::from(69); // 105 in base8
    assert_eq!(
        Base {
            number: 69,
            exponents: vec![
                (Multiplier(1), Power(2)),
                (Multiplier(0), Power(1)), // This should be here but our previous rule cuts this out so we fixed that in line 80
                (Multiplier(5), Power(0))
            ],
            reduced: false
        },
        _69
    );
    let _10 = Base::<8>::from(10);
    assert_eq!(
        Base {
            number: 10,
            exponents: vec![(Multiplier(1), Power(1)), (Multiplier(2), Power(0))],
            reduced: false
        },
        _10
    );
    let _8 = Base::<8>::from(8);
    assert_eq!(
        Base {
            number: 8,
            exponents: vec![(Multiplier(1), Power(1)), (Multiplier(0), Power(0))], // we might have to consider rewriting our library
            reduced: true
        },
        _8
    );
}
#[test]
fn check_base3() {
    let three = Base::<3>::from(11);
    assert_eq!(
        Base {
            number: 11,
            exponents: vec![(Multiplier(1), Power(2)), (Multiplier(2), Power(0))],
            reduced: false
        },
        three
    );
    // println!("{:?}", three);
    let three_into = Into::<u32>::into(three);
    assert_eq!(three_into, 11);
}
#[test]
fn check_base2() {
    let eleven_two = Base::<2>::from(11);
    assert_eq!(
        Base {
            number: 11, // 1011_base2
            exponents: vec![
                (Multiplier(1), Power(3)),
                (Multiplier(1), Power(1)),
                (Multiplier(1), Power(0))
            ],
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
    let eleven_two = Base::<2>::from(11);
    let eleven_three = Base::<3>::from(11);
    let eleven_four = Base::<4>::from(11);
    // println!("Base 2 -------> {eleven_two:?}");
    // println!("Base 3 -------> {eleven_three:?}");
    // println!("Base 4 -------> {eleven_four:?}");
    assert_eq!(eleven_two.compute(), eleven_three.compute());
    assert_eq!(eleven_two.compute(), eleven_four.compute());
    assert_eq!(eleven_two.compute(), eleven_four.number);
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
