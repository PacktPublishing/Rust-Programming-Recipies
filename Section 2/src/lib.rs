pub mod gen_range;
pub mod combi;
pub mod into;
pub struct RangeIterator {
    curr: i32,
    stop: i32,
    step: i32,
}

impl RangeIterator {
    pub fn new(start: i32, stop: i32, step: i32) -> Self {
        RangeIterator {
            curr: start,
            stop,
            step,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut m = 0;
        let it = RangeIterator::new(5, 12, 3);
        for s in it {
            m += s;
        }
        assert_eq!(m, 5 + 8 + 11);
    }
}
