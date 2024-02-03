use crate::context;
use std::io;

#[derive(Debug)]
pub struct Reminder {
    pub text: String,
    pub context: context::Context,
}

impl Reminder {
    pub fn try_new(text: &str) -> io::Result<Self> {
        Ok(Reminder {
            text: text.to_string(),
            context: context::Context::from_current_dir()?,
        })
    }
}
