//! Base<K> is our implementation for a hereditary base notation to be used in computing Goodstein sequences
#![allow(non_snake_case)]
#[cfg(test)]
mod tests;
#[derive(Debug, Default, Clone)]
/// Hereditary base K notation
/// Any number `N` can be represented as a `Sum(A_i.K^i)`
pub struct Base<const K: u32> {
    // The actual number being stored
    pub number: u32,
    // Given exponents and a base K, any number can be expressed,
    pub exponents: Vec<(Multiplier, Power)>,
    // Set true if the maximum exponent produced is equal to or less than K
    pub reduced: bool,
}
impl<const K:u32> Base<K> {
    pub fn compute(&self) -> u32 {
        if self.reduced {
            return self.number;
        }
        self.exponents.iter().fold(0, |acc, e| {
            // TODO: We need both `Multiplier` and `Power` in our exponents list for computing a `Base<K>`.
            // Therefore it would be better to change exponents from `Vec<Base<K>>` to a `Vec<(Multiplier, Power)>`
            // And `Multiplier(u32)` to `Multiplier(Base<K>)` and the same for `Power` respectively.
            
        })
    }
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
            if let Some((perfect_power, power)) = is_power_of(div, K) {
                if perfect_power {
                    break exponent_list.push((Multiplier(1), Power(power)));
                } else {
                    let multiplier = div / power;
                    exponent_list.push((Multiplier(multiplier), Power(power)));
                    div /= power;
                    continue;
                }
            } else {
                break exponent_list.push((Multiplier(div), Power(0)));
            }
        }
        let exponents = exponent_list
            .into_iter()
            .map(|(multi, power)| (multi, power))
            .collect::<Vec<_>>();
        if exponents.len() == 1
            && exponents
                .iter()
                .any(|&exp| match exp.1 {
                    Power(x) if x <= K => true, _ => false
                })
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

/// Returns Some(true, exponent) if `number` is a perfect power of a given `base`
/// else Some(false, exponent) where `number - base.pow(exponent) < base`
/// None is returned if `number < base`
pub fn is_power_of(number: u32, base: u32) -> Option<(bool, u32)> {
    if number < base {
        return None;
    }
    let initial_guess: u32 = 1;
    let mut guess = initial_guess;
    loop {
        let chunk = base.pow(guess);
        if number > chunk && number - chunk > base {
            guess += 1;
        } else if number > chunk && number - chunk < base {
            return Some((false, guess));
        }
        if number < chunk && number - chunk > base {
            return Some((false, guess - 1));
        }
        if number == chunk {
            return Some((true, guess));
        }
    }
}
