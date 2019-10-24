pub struct SkipIterator<I: Iterator> {
    inner: I,
}

impl<I, T> Iterator for SkipIterator<I>
where
    I: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()?;
        self.inner.next()
    }
}

pub trait IterCombi: Iterator + Sized {
    fn skip_half(self) -> SkipIterator<Self> {
        SkipIterator { inner: self }
    }
}

impl<I: Iterator + Sized> IterCombi for I {}

#[cfg(test)]
mod test_skip {
    use super::*;
    #[test]
    fn test_skip_half() {
        let v: i32 = (0..10).skip_half().sum();
        assert_eq!(v, 1 + 3 + 5 + 7 + 9);
    }

    #[test]
    fn test_step_by() {
        let v: i32 = (0..10).step_by(3).sum();
        assert_eq!(v, 0 + 3 + 6 + 9);
    }

    use itertools::Itertools;
    #[test]
    fn test_interleave() {
        let v: Vec<i32> = (0..4).interleave((11..15).rev()).collect();
        assert_eq!(v, vec![0, 14, 1, 13, 2, 12, 3, 11]);
    }

    #[test]
    fn test_intersperse() {
        let s = "hello world etc";
        let v: Vec<&str> = s.split(" ").intersperse(",").collect();
        assert_eq!(v, vec!["hello", ",", "world", ",", "etc"]);

        let s2 = s.split(" ").join(",");
        assert_eq!(s2, "hello,world,etc");
    }
}
