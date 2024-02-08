use crate::context;
use std::io;

#[derive(Debug)]
pub struct Reminder {
    pub text: String,
    pub context: context::Context,
}

impl TryFrom<&str> for Reminder {
    type Error = io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Reminder {
            text: value.to_string(),
            context: context::Context::from_current_dir()?,
        })
    }
}
