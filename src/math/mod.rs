pub trait Monoid {
    type T: Clone + PartialEq;
    fn apply(l: &Self::T, r: &Self::T) -> Self::T;
    fn ident() -> Self::T;
}

pub fn power<M: Monoid>(a: &M::T, mut n: usize) -> M::T {
    let mut acc = M::ident();
    let mut mult = a.clone();
    while n > 0 {
        if n & 1 == 1 {
            acc = M::apply(&acc, &mult);
        }
        mult = M::apply(&mult, &mult);
        n >>= 1;
    }
    acc
}

#[cfg(test)]
mod test {
    use super::*;

    struct Mult;

    impl Monoid for Mult {
        type T = u64;
    
        fn apply(l: &u64, r: &u64) -> u64 {
            l * r % 1_000_000_009
        }
    
        fn ident() -> u64 {
            1
        }
    }
    
    #[test]
    fn test_modular_power() {
        assert_eq!(844428231, power::<Mult>(&2, 63));
    }
}
