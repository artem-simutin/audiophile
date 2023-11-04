use std::fmt::Debug;

pub struct AudiophileError {
    pub location: &'static str,
    pub message: &'static str,
    // pub cause: Option<Box<dyn Error>>,
}

impl Debug for AudiophileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // "[{}] {}\n{:?}",
            "[{}] {}\n",
            self.location,
            self.message,
            // self.cause
        )
    }
}

impl Default for AudiophileError {
    fn default() -> Self {
        println!("WARNING: do not use standard audiophile errors");
        Self {
            location: "[not specified]",
            message: "Something went wrong",
            // cause: None,
        }
    }
}

impl ToString for AudiophileError {
    fn to_string(&self) -> String {
        self.message.to_string()
    }
}
