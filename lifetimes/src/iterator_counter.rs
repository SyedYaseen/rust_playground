struct Counter {
    n: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Counter { n: 0, max: max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.max {
            let res = self.n;
            self.n += 1;
            Some(res)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let c = Counter::new(5);
    assert_eq!(c.collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
}
