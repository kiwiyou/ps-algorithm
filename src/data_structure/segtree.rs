pub trait Operation {
    type Value: Clone;
    type Update;

    fn combine(l: &Self::Value, r: &Self::Value) -> Self::Value;
    fn i_value() -> Self::Value;
    fn apply(v: &mut Self::Value, u: &Self::Update);
}

pub struct SegmentTree<O: Operation> {
    val: Vec<O::Value>,
    n: usize,
}

impl<O: Operation> SegmentTree<O> {
    pub fn new(n: usize, data: impl IntoIterator<Item = O::Value>) -> Self {
        let mut val = vec![O::i_value(); n];
        val.extend(data.into_iter().take(n));
        for i in (1..n).rev() {
            val[i] = O::combine(&val[i << 1], &val[i << 1 | 1]);
        }
        Self { val, n }
    }

    pub fn modify(&mut self, mut i: usize, amount: &O::Update) {
        i += self.n;
        O::apply(&mut self.val[i], amount);
        while i > 1 {
            i >>= 1;
            self.val[i] = O::combine(&self.val[i << 1], &self.val[i << 1 | 1]);
        }
    }

    pub fn query(&mut self, mut l: usize, mut r: usize) -> O::Value {
        l += self.n;
        r += self.n;
        let mut result = O::i_value();
        while l < r {
            if l & 1 == 1 {
                result = O::combine(&result, &self.val[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                result = O::combine(&self.val[r], &result);
            }
            l >>= 1;
            r >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct SumSet;
    impl Operation for SumSet {
        type Value = i64;
        type Update = i64;
        fn combine(l: &Self::Value, r: &Self::Value) -> Self::Value {
            l + r
        }
        fn i_value() -> Self::Value {
            0
        }
        fn apply(v: &mut Self::Value, u: &Self::Update) {
            *v = *u;
        }
    }

    #[test]
    fn query() {
        let mut tree = SegmentTree::<SumSet>::new(10, [2, 1, 4, 3, 6, 5, 8, 7, 10, 9]);
        assert_eq!(10, tree.query(0, 4));
        assert_eq!(48, tree.query(3, 10));
    }

    #[test]
    fn modify() {
        let mut tree = SegmentTree::<SumSet>::new(10, [2, 1, 4, 3, 6, 5, 8, 7, 10, 9]);
        tree.modify(5, &2);
        assert_eq!(
            vec![0, 52, 37, 15, 34, 3, 7, 8, 15, 19, 2, 1, 4, 3, 6, 2, 8, 7, 10, 9],
            tree.val
        );
    }
}
