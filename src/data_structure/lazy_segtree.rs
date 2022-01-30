pub trait Operation {
    type Value: Clone;
    type Update: Clone;

    fn combine(l: &Self::Value, r: &Self::Value) -> Self::Value;
    fn i_value() -> Self::Value;
    fn compose(a: &mut Self::Update, b: &Self::Update);
    fn is_i(u: &Self::Update) -> bool;
    fn i_update() -> Self::Update;
    fn apply(v: &mut Self::Value, len: usize, u: &Self::Update);
}

pub struct LazySegmentTree<O: Operation> {
    val: Vec<O::Value>,
    lazy: Vec<O::Update>,
    n: usize,
    h: u32,
}

impl<O: Operation> LazySegmentTree<O> {
    pub fn new(n: usize, data: impl IntoIterator<Item = O::Value>) -> Self {
        let mut val = vec![O::i_value(); n];
        val.extend(data);
        let lazy = vec![O::i_update(); n];
        let h = 64 - n.leading_zeros();
        for i in (1..n).rev() {
            val[i] = O::combine(&val[i << 1], &val[i << 1 | 1]);
        }
        Self { val, lazy, n, h }
    }

    fn apply(&mut self, i: usize, len: usize, amount: &O::Update) {
        O::apply(&mut self.val[i], len, amount);
        if i < self.n {
            O::compose(&mut self.lazy[i], amount);
        }
    }

    fn update(&mut self, mut i: usize) {
        let mut len = 2;
        while i > 1 {
            i >>= 1;
            self.val[i] = O::combine(&self.val[i << 1], &self.val[i << 1 | 1]);
            if !O::is_i(&self.lazy[i]) {
                O::apply(&mut self.val[i], len, &self.lazy[i]);
            }
            len <<= 1;
        }
    }

    fn propagate(&mut self, i: usize) {
        let mut len = 1 << (self.h - 1);
        for shift in (1..=self.h).rev() {
            let ptr = i >> shift;
            if !O::is_i(&self.lazy[ptr]) {
                self.apply(ptr << 1, len, &self.lazy[ptr].clone());
                self.apply(ptr << 1 | 1, len, &self.lazy[ptr].clone());
                self.lazy[ptr] = O::i_update();
            }
            len >>= 1;
        }
    }

    pub fn modify(&mut self, mut l: usize, mut r: usize, amount: &O::Update) {
        l += self.n;
        r += self.n;
        let mut lp = l;
        let mut rp = r;
        self.propagate(l);
        if l < r {
            self.propagate(r - 1);
        }
        let mut len = 1;
        while lp < rp {
            if lp & 1 == 1 {
                self.apply(lp, len, amount);
                lp += 1;
            }
            if rp & 1 == 1 {
                rp -= 1;
                self.apply(rp, len, amount);
            }
            lp >>= 1;
            rp >>= 1;
            len <<= 1;
        }
        self.update(l);
        if l < r {
            self.update(r - 1);
        }
    }

    pub fn query(&mut self, l: usize, r: usize) -> O::Value {
        let mut lp = l + self.n;
        let mut rp = r + self.n;
        self.propagate(lp);
        if l < r {
            self.propagate(rp - 1);
        }
        let mut result = O::i_value();
        while lp < rp {
            if lp & 1 == 1 {
                result = O::combine(&result, &self.val[lp]);
                lp += 1;
            }
            if rp & 1 == 1 {
                rp -= 1;
                result = O::combine(&self.val[rp], &result);
            }
            lp >>= 1;
            rp >>= 1;
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
        fn compose(a: &mut Self::Update, b: &Self::Update) {
            *a += b
        }
        fn is_i(u: &Self::Update) -> bool {
            *u == 0
        }
        fn i_update() -> Self::Update {
            0
        }
        fn apply(v: &mut Self::Value, len: usize, u: &Self::Update) {
            *v += len as i64 * u;
        }
    }

    #[test]
    fn query() {
        let mut tree = LazySegmentTree::<SumAdd>::new(10, [2, 1, 4, 3, 6, 5, 8, 7, 10, 9]);
        assert_eq!(10, tree.query(0, 4));
        assert_eq!(48, tree.query(3, 10));
    }

    #[test]
    fn modify() {
        let mut tree = LazySegmentTree::<SumAdd>::new(10, [2, 1, 4, 3, 6, 5, 8, 7, 10, 9]);
        tree.modify(5, 9, &-5);
        assert_eq!(
            vec![0, 35, 22, 13, 19, 3, 7, 6, 5, 14, 2, 1, 4, 3, 6, 0, 8, 7, 5, 9],
            tree.val,
        );
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, -5, 0], tree.lazy);
    }
}
