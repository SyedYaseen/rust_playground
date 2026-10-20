pub struct StrSplit<'s, D> {
    haystack: Option<&'s str>,
    delimiter: D,
}

impl<'s, D> StrSplit<'s, D> {
    pub fn new(s: &'s str, delimiter: D) -> Self {
        Self {
            haystack: Some(s),
            delimiter: delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

impl<'s, D> Iterator for StrSplit<'s, D>
where
    D: Delimiter,
{
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.haystack {
            if let Some((start, end)) = self.delimiter.find_next(remainder) {
                let text_until = &remainder[..start];
                *remainder = &remainder[end..];
                Some(text_until)
            } else {
                self.haystack.take()
            }
        } else {
            None
        }
    }
}

fn until_char<'s>(s: &'s str, ch: char) -> &'s str {
    StrSplit::new(s, ch).next().expect("Will always have value")
}

#[test]
fn split() {
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
    // let unti = until_char(&input, &format!("o"));
    let unti = until_char(&input, 'o');
    assert_eq!(unti, "hell");
}
