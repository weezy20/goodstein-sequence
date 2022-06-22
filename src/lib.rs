//! Base<K> is our implementation for a hereditary base notation to be used in computing Goodstein sequences
use std::fmt::Display;
static DIGITS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";
#[cfg(test)]
mod tests;
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Multiplier(u32);
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Power(u32);
#[derive(Debug, Default, Clone, PartialEq)]
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
impl<const K: u32> Base<K> {
    /// Build a number from exponent list
    pub fn compute(&self) -> u32 {
        let s: u32 = self
            .exponents
            .iter()
            .fold(0, |acc, &x| acc + x.0 .0 * K.pow(x.1 .0));
        assert_eq!(s, self.number);
        s
    }
}

// Tuple struct constructors are first-class functions, like variant constructors.
// So the declaration of a tuple struct injects a value into the value namespace. The variable in an expression like x() looks for a name in the value namespace.
// However, the structure name in a structure literal expression like S { ... } (also structure patterns like S { ... }) looks for a name in the type namespace. The typedef declaration type only adds a name to the type namespace, not the value namespace. So it works for named structure literals and patterns, but not for tuple structs.

// Type aliases cannot be used to construct Tuple variants of structs
// https://github.com/rust-lang/rust/issues/17422
// pub type M = Multiplier;
// pub type P = Power;
// M(1) or P(2) won't work

// Note: confusingly these are not overlapping From and Into trait impls, ie :)
// From<A> for B implies Into<B> for A, but not Into<A> for B
impl<const K: u32> Into<u32> for Base<K> {
    fn into(self) -> u32 {
        self.number
    }
}
// TODO: Scrutinize 'reduced' logic
impl<const K: u32> From<u32> for Base<K> {
    // Given any number `n` in base-10 to return a base-k representation of it.
    fn from(n: u32) -> Self {
        assert!(K >= 2, "Bases less than 2 do not make sense");
        assert!(
            K <= (26 + 10),
            "We run out of ascii symbols to represent more than base 36 digits"
        );
        let mut reduced = false;
        //                         (multiplier, power)
        let mut exponent_list: Vec<(Multiplier, Power)> = vec![];
        if n == 0 {
            exponent_list.push((Multiplier(0), Power(0)));
            reduced = !reduced;
            return Base {
                exponents: exponent_list,
                reduced,
                number: n,
            };
        }
        match is_power_of(n, K) {
            Some((true, p)) => exponent_list.push((Multiplier(1), Power(p))),
            Some((false, p)) => {
                let mut power_counter = p;
                let mut div = n;
                loop {
                    // read this condition after loop body
                    if div < K {
                        while power_counter > 0 {
                            // Fill intermediate spaces with 0s
                            exponent_list.push((Multiplier(0), Power(power_counter)));
                            power_counter -= 1;
                        }
                        if div != 0 {
                            break exponent_list.push((Multiplier(div), Power(0)));
                        } else {
                            // This optional else clause is here only for convinience sake when generating
                            // the display impl, as we can simply pick and place while iterating over the list
                            // instead having to write a condition to check if the specific 0 multiplier Power(p) exists
                            // in our list
                            break exponent_list.push((Multiplier(0), Power(0)));
                        }
                    }
                    let mult = div / K.pow(power_counter);
                    let rem = div % K.pow(power_counter);
                    let entry = (Multiplier(mult), Power(power_counter));
                    exponent_list.push(entry);
                    power_counter -= 1;
                    div = rem;
                }
            }
            None => exponent_list.push((Multiplier(n), Power(0))),
        }
        Base {
            exponents: exponent_list,
            reduced,
            number: n,
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
        if number == chunk {
            return Some((true, guess));
        } else if number > chunk {
            if number >= chunk * base {
                guess += 1;
            } else {
                return Some((false, guess));
            }
        } else if number < chunk {
            return Some((false, guess - 1));
        }
    }
}

impl<const K: u32> Display for Base<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: Vec<char> = vec![];
        let digits = DIGITS.chars().collect::<Vec<char>>();
        let mut power_counter = 0;
        self.exponents.iter().for_each(|(m, p)| {
            power_counter = p.0;
            result.push(digits[m.0 as usize]);
        });
        while power_counter != 0 {
            result.push(digits[0]);
            power_counter -= 1;
        }
        for c in result {
            c.fmt(f)?;
        }
        Ok(())
    }
}
