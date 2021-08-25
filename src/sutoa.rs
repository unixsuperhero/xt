use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sutoa<T>
where
    T: Hash + Eq + Clone,
{
    pub list: Vec<T>,
    pub rev: HashMap<T, usize>,
    pub next: usize,
}

impl<T> Sutoa<T>
where
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            list: vec![],
            rev: HashMap::default(),
            next: 0,
        }
    }

    pub fn set(&mut self, val: T) {
        match self.rev.contains_key(&val) {
            true => {}
            false => {
                self.rev.insert(val.to_owned(), self.list.len());
                self.list.push(val.to_owned());
                self.next = self.list.len();
            }
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        if key < self.list.len() {
            Some(&self.list[key])
        } else {
            None
        }
    }

    pub fn get_key(&self, val: T) -> Option<&usize> {
        if let true = self.rev.contains_key(&val) {
            return self.rev.get(&val);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sutoa_new() {
        let s: Sutoa<String> = Sutoa::new();
        assert_eq!(s.next, 0);

        let empty_vec: Vec<String> = vec![];
        assert_eq!(s.list, empty_vec);

        let rev: HashMap<String, usize> = HashMap::default();
        assert_eq!(s.rev, rev);
    }

    #[test]
    fn test_sutoa_set() {
        let mut s: Sutoa<String> = Sutoa::new();
        assert_eq!(s.next, 0);
        assert_eq!(s.list.len(), 0);
        assert_eq!(s.rev.len(), 0);

        s.set("my life".to_string());
        assert_eq!(s.next, 1);
        assert_eq!(s.list.len(), 1);
        assert_eq!(s.rev.len(), 1);

        s.set("your life".to_string());
        s.set("your life".to_string());
        assert_eq!(s.next, 2);
        assert_eq!(s.list.len(), 2);
        assert_eq!(s.rev.len(), 2);

        s.set("your life".to_string());
        s.set("my life".to_string());
        assert_eq!(s.next, 2);
        assert_eq!(s.list.len(), 2);
        assert_eq!(s.rev.len(), 2);
    }

    #[test]
    fn test_sutoa_get() {
        let mut s: Sutoa<String> = Sutoa::new();

        s.set("my life".to_string());
        s.set("no life".to_string());
        s.set("no life".to_string());
        s.set("third life".to_string());
        s.set("no life".to_string());

        let val = s.get(0);
        assert_eq!(val, Some(&"my life".to_string()));

        let val = s.get(1);
        assert_eq!(val, Some(&"no life".to_string()));

        let val = s.get(2);
        assert_eq!(val, Some(&"third life".to_string()));

        let val = s.get(3);
        assert_eq!(val, None);
    }

    #[test]
    fn test_sutoa_get_key() {
        let mut s: Sutoa<String> = Sutoa::new();

        s.set("my life".to_string());
        s.set("no life".to_string());
        s.set("no life".to_string());
        s.set("no life".to_string());
        s.set("third life".to_string());
        s.set("no life".to_string());

        let key = s.get_key("my life".to_string());
        assert_eq!(key, Some(&0));

        let key = s.get_key("no life".to_string());
        assert_eq!(key, Some(&1));

        let key = s.get_key("third life".to_string());
        assert_eq!(key, Some(&2));

        let key = s.get_key("missing string".to_string());
        assert_eq!(key, None);
    }
}
