pub trait Operation {
    type Value: Clone;
    type Update;

    fn combine(l: &Self::Value, r: &Self::Value) -> Self::Value;
    fn i_value() -> Self::Value;
    fn apply(v: &mut Self::Value, u: &Self::Update);
}

pub struct FenwickTree<O: Operation> {
    val: Vec<O::Value>,
    n: usize,
}

impl<O: Operation> FenwickTree<O> {
    pub fn new(n: usize) -> Self {
        Self {
            val: vec![O::i_value(); n + 1],
            n,
        }
    }

    // i: 1-based index
    pub fn modify(&mut self, mut i: usize, amount: &O::Update) {
        while i <= self.n {
            O::apply(&mut self.val[i], amount);
            i = (i | (i - 1)) + 1;
        }
    }

    pub fn query(&mut self, mut count: usize) -> O::Value {
        let mut result = O::i_value();
        while count > 0 {
            result = O::combine(&self.val[count], &result);
            count = count & (count - 1);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct SumAdd;
    impl Operation for SumAdd {
        type Value = i64;
        type Update = i64;
        fn combine(l: &Self::Value, r: &Self::Value) -> Self::Value {
            l + r
        }
        fn i_value() -> Self::Value {
            0
        }
        fn apply(v: &mut Self::Value, u: &Self::Update) {
            *v += *u;
        }
    }

    #[test]
    fn modify() {
        let mut tree = FenwickTree::<SumAdd>::new(10);
        for (i, v) in [2, 1, 4, 3, 6, 5, 8, 7, 10, 9].iter().enumerate() {
            tree.modify(i + 1, &v);
        }
        assert_eq!(vec![0, 2, 3, 4, 10, 6, 11, 8, 36, 10, 19], tree.val);
    }

    #[test]
    fn query() {
        let mut tree = FenwickTree::<SumAdd>::new(10);
        for (i, v) in [2, 1, 4, 3, 6, 5, 8, 7, 10, 9].iter().enumerate() {
            tree.modify(i + 1, &v);
        }
        assert_eq!(10, tree.query(4));
        assert_eq!(55, tree.query(10));
    }
}
