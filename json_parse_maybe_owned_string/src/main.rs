use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::collections::HashMap;

enum SmartString<'a> {
    Owned(String),
    Borrowed(&'a str),
}

enum JsonValue<'a> {
    Object(HashMap<SmartString<'a>, JsonValue<'a>>),
    Array(Vec<JsonValue<'a>>),
    Value(SmartString<'a>),
}

impl<'a> Deref for SmartString<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            SmartString::Owned(s) => s,
            SmartString::Borrowed(s) => s,
        }
    }
}

impl<'a> Hash for SmartString<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self).hash(state)
    }
}

impl<'a> fmt::Display for SmartString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl<'a> SmartString<'a> {
    pub fn new(s: &'a str) -> Self {
        SmartString::Borrowed(s)
    }

    pub fn to_owned(&self) -> Self {
        SmartString::Owned(self.to_string())
    }

    pub fn into_string(self) -> String {
        match self {
            SmartString::Owned(s) => s,
            SmartString::Borrowed(s) => s.to_string(),
        }
    }
}


fn main() {
    let owned = SmartString::Owned("Hello, owned!".to_string());
    let borrowed = SmartString::new("Hello, borrowed!");

    println!("{}", owned); // "Hello, owned!"
    println!("{}", borrowed); // "Hello, borrowed!"
}