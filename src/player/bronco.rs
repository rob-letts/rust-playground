use std::fmt::Display;

use super::{cheer::Cheer, default::Default};

#[derive(Debug)]
pub struct Bronco {
    pub name: String,
    pub id: i32,
}

pub struct BroncoIter {
    index: i32,
    len: i32,
}

impl Iterator for BroncoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        self.index += 1;
        return Some(self.index);
    }
}

// impl IntoIterator for Bronco {
//     type Item = i32;

//     type IntoIter = BroncoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

impl Display for Bronco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Let's go Broncos - {}: {}", &self.name, &self.id);
    }
}

impl Default for Bronco {
    fn default() -> Self {
        return Bronco {
            name: "Default Name".to_string(),
            id: 0,
        };
    }
}

impl Cheer for Bronco {
    fn lets_go(&self) {
        println!("{}", &self);
    }
}
