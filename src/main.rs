extern crate libc;
extern crate regex;
extern crate memmap;

#[cfg(feature = "pcre")]
mod pcre;
#[cfg(feature = "pcre")]
use pcre::Regex;

#[cfg(not(feature = "pcre"))]
use regex::bytes::Regex;

struct Lines<'a>(&'a [u8]);

impl<'a> Iterator for Lines<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        if self.0.is_empty() {
            return None;
        }
        let mut split = self.0.splitn(2, |&x| x == b'\n');
        let line = split.next();
        self.0 = split.next().unwrap_or(&[]);
        line
    }
}

fn main() {
    let regex = std::env::args().nth(1).unwrap();
    let path = std::env::args().nth(2).unwrap();

    let regex = Regex::new(&regex).unwrap();
    let map = memmap::Mmap::open_path(&path, memmap::Protection::Read).unwrap();
    let buf = unsafe { map.as_slice() };

    let mut found = 0;
    for line in Lines(buf) {
        if let Some(span) = regex.find(line) {
            let mut searchfrom = span.1;
            found += 1;
            while let Some((_, end)) = regex.find(&line[searchfrom..]) {
                found += 1;
                searchfrom += end;
            }
        }
    }
    println!("{}", found);
}
