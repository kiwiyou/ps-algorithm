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
}
