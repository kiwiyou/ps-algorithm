// First element of z array is 0
pub fn z<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let mut l = 0;
    let mut r = 0;
    let mut z = vec![0];
    for i in 1..s.len() {
        let new_z = if i >= r {
            let eq_len = s[i..]
                .iter()
                .zip(s)
                .position(|(a, b)| a != b)
                .unwrap_or(s.len() - i);
            l = i;
            r = l + eq_len;
            eq_len
        } else if i + z[i - l] >= r {
            let eq_len = s[r..]
                .iter()
                .zip(&s[r - l..])
                .position(|(a, b)| a != b)
                .unwrap_or(s.len() - r);
            l = i;
            r += eq_len;
            r - l
        } else {
            z[i - l]
        };
        z.push(new_z);
    }
    z
}

pub struct KmpMatcher<'a, T> {
    pi: Vec<usize>,
    pattern: &'a [T],
}

impl<'a, T: PartialEq> KmpMatcher<'a, T> {
    // |pattern| > 0
    pub fn new(pattern: &'a [T]) -> Self {
        let mut pi = vec![0];
        let mut j = 0;
        for i in 1..pattern.len() {
            while j > 0 && pattern[i] != pattern[j] {
                j = pi[j - 1];
            }
            if pattern[i] == pattern[j] {
                j += 1;
            }
            pi.push(j);
        }
        Self { pi, pattern }
    }

    pub fn find<I: IntoIterator<Item = &'a T>>(&self, s: I) -> Kmp<I::IntoIter, T> {
        Kmp {
            matcher: self,
            i: s.into_iter(),
            j: 0,
        }
    }
}

pub struct Kmp<'a, I, T: PartialEq> {
    matcher: &'a KmpMatcher<'a, T>,
    i: I,
    j: usize,
}

impl<'a, I: Iterator<Item = &'a T>, T: PartialEq> Iterator for Kmp<'a, I, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let si = self.i.next()?;
        while self.j > 0 && self.matcher.pattern.get(self.j) != Some(si) {
            self.j = self.matcher.pi[self.j - 1];
        }
        if &self.matcher.pattern[self.j] == si {
            self.j += 1;
        }
        Some(self.j)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_z() {
        assert_eq!(
            vec![0, 0, 3, 0, 1, 3, 0, 1, 5, 0, 3, 0, 1],
            z("ababaabaababa".as_bytes()),
        );
    }

    #[test]
    fn test_pi() {
        assert_eq!(
            vec![0, 1, 0, 1, 2, 3, 0, 1, 0, 0, 1, 0, 1],
            KmpMatcher::new("aabaabbabbaba".as_bytes()).pi
        )
    }

    #[test]
    fn test_kmp() {
        let matcher = KmpMatcher::new("aba".as_bytes());
        let result: Vec<usize> = matcher.find("aabaabababa".as_bytes()).collect();
        assert_eq!(vec![1, 1, 2, 3, 1, 2, 3, 2, 3, 2, 3], result);
    }
}
