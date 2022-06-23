//! We follow the design of `Base<K>` but deviate on important grounds such as replacing Power(UnsignedInteger) with Power(Base<K>)
use crate::{Base, Multiplier, UnsignedInteger};

/// Generate a Goodstein sequence from a `Base<K>` number such that the following operations are defined
/// 1. Bump base of all nested powers such that `GoosteinSeq<K>` turns to `GoosteinSeq<K+1>`
/// 2. Substract one from the result of `compute` on `GoodsteinSeq<K>`
#[derive(Debug, PartialEq, Clone)]
pub struct GoodsteinSeq<const K: UnsignedInteger> {
    pub base_number: Base<K>,
    pub g_exponents: Vec<(Multiplier, GPow<K>)>,
    pub reduced: bool,
}
impl<const K: UnsignedInteger> From<Base<K>> for GoodsteinSeq<K> {
    fn from(n: Base<K>) -> Self {
        let exponents: Vec<(Multiplier, GPow<K>)> = n
            .exponents
            .iter()
            .map(|(m, p)| {
                (
                    m.to_owned(),
                    if p.0 > K {
                        GPow::NonReduced(Base::<K>::from(p.0))
                    } else {
                        GPow::Reduced(p.0)
                    },
                )
            })
            .collect();

        GoodsteinSeq {
            base_number: n.clone(),
            g_exponents: exponents,
            reduced: n.reduced,
        }
    }
}
#[derive(Clone, PartialEq)]
/// The exponent maybe either > K or <= K (reduced). For non reduced cases, we want to expand it into
/// it's own `exponent_list` which is why we use Base<K> to represent non-reduced exponents
pub enum GPow<const K: UnsignedInteger> {
    Reduced(UnsignedInteger),
    NonReduced(Base<K>),
}
impl<const K: UnsignedInteger> std::fmt::Debug for GPow<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reduced(arg0) => f.debug_tuple("Reduced").field(arg0).finish(),
            Self::NonReduced(arg0) => f
                .debug_tuple("NonReduced")
                .field(&arg0.number)
                .field(&arg0.reduced)
                .field(arg0)
                .finish(),
        }
    }
}
impl<const K: UnsignedInteger> std::fmt::Display for GoodsteinSeq<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<const K: UnsignedInteger> GoodsteinSeq<K> {
    pub fn get_num(&self) -> UnsignedInteger {
        self.base_number.number
    }
    pub fn get_compute(&self) -> UnsignedInteger {
        self.base_number.compute()
    }
    pub fn bump_base(&mut self) {
        todo!()
    }
    pub fn substract_one(&mut self) {
        todo!()
    }
}
