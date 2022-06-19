use super::*;
#[test]
fn check_init()
{
    let three = Base::<3>::from(11);
    println!("{:#?}", three);
}