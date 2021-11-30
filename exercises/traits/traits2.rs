// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
use std::ops::Add;

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

trait AppendEachBar {
    fn append_each(self) -> Self;
}

impl AppendEachBar for Vec<String> {
    fn append_each(mut self) -> Self {
        for s in self.iter_mut() {
            s.push_str("Bar");
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

    #[test]
    fn test_append_each() {
        let mut foo = vec![String::from("Foo")].append_each();
        assert_eq!(foo.pop().unwrap(), String::from("FooBar"));
    }
}
