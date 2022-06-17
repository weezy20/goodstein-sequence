#[derive(Debug, Default, Clone)]
pub struct Base<const K: u32> {
    pub exponents: Exponent,
}
#[derive(Debug, Default, Clone)]
pub struct Exponent {
    pub list: Vec<u32>,
}
impl<const K: u32> Base<K> {
    fn compute(&self) -> u32 {
        self.exponents
            .list
            .iter()
            .fold(0, |acc, exp| acc + K.pow(*exp))
    }
}
impl<const K: u32> From<u32> for Base<K> {
    // Compute a base-2 goodstein sequence
    fn from(n: u32) -> Self {
        let binary_n = format!("{n:b}");
        let mut list: Vec<u32> = vec![];
        let mut count: u32 = 0;
        binary_n.chars().rev().for_each(|ch| {
            if ch == '1' {
                list.push(count);
            }
            count += 1;
        });
        Self {
            exponents: Exponent {
                list: list.into_iter().rev().collect::<Vec<u32>>(),
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_100_goodstein_sequence() {
        // 100 = 2^6 + 2^5 + 2^2
        // therefore exponents should be [6,5,2]
        const N: u32 = 100;
        let bn = Base::<N>::from(N);
        let list = bn.exponents.list.clone();
        assert_eq!(vec![6_u32, 5, 2], list);
    }
}
