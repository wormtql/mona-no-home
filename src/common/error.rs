use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct MyStringError {
    pub msg: String
}

impl Display for MyStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for MyStringError {

}

// impl<T: Error> From<T> for MyStringError {
//     fn from(e: T) -> Self {
//         MyStringError {
//             msg: String::from(&e.to_string())
//         }
//     }
// }

impl MyStringError {
    pub fn from(s: &str) -> Self {
        Self {
            msg: String::from(s)
        }
    }
}