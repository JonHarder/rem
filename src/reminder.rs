use crate::context;

#[derive(Debug)]
pub struct Reminder {
    pub text: String,
    pub context: context::Context,
}

impl Reminder {
    pub fn new(text: &str) -> Self {
        Reminder {
            text: text.to_string(),
            context: context::Context::new(),
        }
    }
}
