//! Base<K> is our implementation for a hereditary base notation to be used in computing Goodstein sequences
#![allow(non_snake_case)]
#[cfg(test)]
mod tests;
#[derive(Debug, Default, Clone)]
/// Hereditary base K notation
pub struct Base<const K: u32> {
    // The actual number being stored
    pub number: u32,
    // Given exponents and a base K, any number can be expressed,
    pub exponents: Vec<Base<K>>,
    // Set true if the maximum exponent produced is equal to or less than K
    pub reduced: bool,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Multiplier(u32);
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Power(u32);

// Tuple struct constructors are first-class functions, like variant constructors.
// So the declaration of a tuple struct injects a value into the value namespace. The variable in an expression like x() looks for a name in the value namespace.
// However, the structure name in a structure literal expression like S { ... } (also structure patterns like S { ... }) looks for a name in the type namespace. The typedef declaration type only adds a name to the type namespace, not the value namespace. So it works for named structure literals and patterns, but not for tuple structs.

// Type aliases cannot be used to construct Tuple variants of structs
// https://github.com/rust-lang/rust/issues/17422
// pub type M = Multiplier;
// pub type P = Power;
// M(1) or P(2) won't work

impl<const K: u32> From<u32> for Base<K> {
    // Given any number `n` in base-10 to return a base-k representation of it.
    fn from(n: u32) -> Self {
        let mut reduced = false;
        //                         (multiplier, power)
        let mut exponent_list: Vec<(Multiplier, Power)> = vec![];
        let mut div = n;
        let mut initial_power = 0;
        loop {
            // Stackoverflow if true as we need a limiting case for Base<K> calling from indefinitely
            if div == 0 {
                break reduced = true;
            }
            if div / K == 1 {
                break exponent_list.push((Multiplier(1), Power(1)));
            } else if div / K < 1 {
                break exponent_list.push((Multiplier(1), Power(0)));
            }
            // Check if number is a power of K itself
            if let Some(power) = is_power_of(div, K) {
                break exponent_list.push((Multiplier(1), Power(power)));
            }
            exponent_list.push((
                Multiplier(todo!("Find appropriate multipler and power")),
                Power(todo!("Find appropriate multipler and power")),
            ));
            div /= K;
        }
        let exponents = exponent_list
            .into_iter()
            .map(|(_multi, power)| Base::<K>::from(power.0))
            .collect::<Vec<Base<K>>>();
        if exponents.len() == 1
            && exponents
                .iter()
                .any(|exp| exp.number == 1 || exp.number == 0)
        {
            reduced = true;
        }
        Base {
            number: n,
            exponents,
            reduced,
        }
    }
}

/// Returns Some(exponent) if `number` is a power of a given base `power_of`
pub fn is_power_of(number: u32, power_of: u32) -> Option<u32> {
    None
}
