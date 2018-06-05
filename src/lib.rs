extern crate chrono;
extern crate serde;

use chrono::prelude::{Date, DateTime, NaiveDate, Utc};

use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
pub struct Key(pub Vec<u8>);

impl Key {
    pub fn join(&self, k: Key) -> Key {
        let mut left: Vec<u8> = self.0.clone();
        let mut right: Vec<u8> = k.0;
        left.append(&mut right);
        Key(left)
    }

    pub fn empty() -> Key {
        let v: Vec<u8> = Vec::new();
        Key(v)
    }
}

impl From<Key> for Vec<u8> {
    fn from(val: Key) -> Self {
        val.0
    }
}

impl<'a> From<&'a str> for Key {
    fn from(val: &str) -> Self {
        Key(val.as_bytes().to_vec())
    }
}

impl From<String> for Key {
    fn from(val: String) -> Self {
        Key(val.as_bytes().to_vec())
    }
}

impl From<DateTime<Utc>> for Key {
    fn from(val: DateTime<Utc>) -> Self {
        Key(val.to_rfc3339().as_bytes().to_vec())
    }
}

impl From<Box<[u8]>> for Key {
    fn from(val: Box<[u8]>) -> Self {
        Key(val.to_vec())
    }
}

impl From<Date<Utc>> for Key {
    /// Here from adds in zero-values for hour, minute, second to let us work with a DateTime.
    ///
    /// We require a DateTime for it's rfc3339 representation.
    fn from(val: Date<Utc>) -> Self {
        Key(val.and_hms(0, 0, 0).to_rfc3339().as_bytes().to_vec())
    }
}

impl From<NaiveDate> for Key {
    fn from(val: NaiveDate) -> Self {
        let dt = Date::<Utc>::from_utc(val, Utc);
        Key(dt.and_hms(0, 0, 0).to_rfc3339().as_bytes().to_vec())
    }
}

/// Users of this macro must have keyz::Key in scope.
#[macro_export]
macro_rules! make_key {
    () => (
        Key::empty()
    );
    ($x:expr) => (
        Key::from($x)
    );
    ($x:expr $(, $more:expr)*) => (
        Key::join(&Key::from($x), make_key!($($more),*))
    );
}

#[cfg(test)]
mod test {
    use chrono::prelude::*;
    #[test]
    fn test_make_key() {
        use super::*;
        use std::convert::From;

        // &str -> Key
        let key = make_key!("hello", String::from("world"));
        let expected: Vec<u8> = Vec::from("helloworld".as_bytes());
        assert_eq!(key, Key(expected));

        // Date -> Key
        let date = Utc.ymd(2016, 11, 8);
        let key2 = make_key!(date, "hello");
        let expected2 = make_key!(date.and_hms(0, 0, 0), "hello");
        assert_eq!(key2, expected2);
    }
}
