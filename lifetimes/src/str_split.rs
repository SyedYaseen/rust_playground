#[derive(Debug)]
pub struct StrSplit<'rem, 'delim> {
    rem: Option<&'rem str>,
    delim: &'delim str,
}

impl<'rem, 'delim> StrSplit<'rem, 'delim> {
    pub fn new(haystack: &'rem str, delimiter: &'delim str) -> Self {
        Self {
            rem: Some(haystack),
            delim: delimiter,
        }
    }
}

impl<'rem, 'delim> Iterator for StrSplit<'rem, 'delim> {
    type Item = &'rem str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.rem {
            if let Some(next_delim) = remainder.find(self.delim) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[next_delim + self.delim.len()..];
                Some(until_delim)
            } else {
                self.rem.take()
            }
        } else {
            None
        }
    }
}

fn until_char<'s>(s: &'s str, c: &str) -> &'s str {
    StrSplit::new(s, c)
        .next()
        .expect("Next will atleast return empty str")
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");

    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");

    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}

#[test]
fn get_until_char() {
    let input = "hello";
    let unti = until_char(&input, &format!("o"));
    assert_eq!(unti, "hell");
}
