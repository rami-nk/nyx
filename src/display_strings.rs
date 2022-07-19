use std::fmt::Display;
use colored::*;

pub struct DisplayStrings {
    strings: Vec<String>,
    offset: usize,
    color: String,
}

impl DisplayStrings {
   pub fn new(offset: usize, color: &str) -> Self {
       Self { strings: Vec::new(), offset: offset, color: String::from(color) }
   } 
   
   pub fn push(&mut self, string: &str) {
       self.strings.push(string.to_string());
   }
   
   pub fn try_print_with_prefix(&self, prefix: &str) {
       if self.is_not_empty() {
           println!("{}", prefix);
           println!("{}", self);
       }
   }
   
   pub fn is_not_empty(&self) -> bool {
       !self.strings.is_empty()
   }

   pub fn is_empty(&self) -> bool {
       self.strings.is_empty()
   }
}

impl Display for DisplayStrings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.strings.iter().fold(Ok(()), |result, string| {
            result.and_then(|_| writeln!(f, "{}{}", " ".repeat(self.offset), string.as_str().color(&*self.color)))
        })
    }
}
