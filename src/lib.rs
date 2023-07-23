pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
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

// #[test] 
// fn empty_tail() {
//     let haystack = "a b c d ";
//     let letters = StrSplit::new(haystack, " ");
//     assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
// }
