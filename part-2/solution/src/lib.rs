use std::cell::RefCell;

pub struct StringStore {
    strings: RefCell<Vec<String>>,
}

impl StringStore {
    pub fn with_capacity(num_strings: usize, string_len: usize) -> StringStore {
        let mut strings = Vec::with_capacity(num_strings);
        for _ in 0..num_strings {
            strings.push(String::with_capacity(string_len))
        }

        StringStore {
            strings: RefCell::new(strings),
        }
    }

    pub fn get<'a>(&'a self) -> Guard<'a> {
        let mut strings = self.strings.borrow_mut();
        let string = match strings.pop() {
            Some(string) => string,
            None => String::new(),
        };
        Guard {
            string: Some(string),
            store: self,
        }
    }
}

pub struct Guard<'a> {
    string: Option<String>,
    store: &'a StringStore,
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        if let Some(mut string) = self.string.take() {
            string.clear();
            self.store.strings.borrow_mut().push(string)
        }
    }
}

use std::ops::{Deref, DerefMut};

impl<'a> Deref for Guard<'a> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self.string.as_ref().unwrap()
    }
}

impl<'a> DerefMut for Guard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.string.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let store = StringStore::with_capacity(1, 3);
        let mut string = store.get();

        string.push_str("Wow");
        drop(string);

        let string = store.get();

        assert_eq!(string.len(), 0);
    }
}
