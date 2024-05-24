struct ErrorReporter {
    errors: Vec<Error>,
}

pub struct Error {
    message: String,
    location: (u32, u32),
}

impl Error {
    pub fn new(message: String, location: (u32, u32)) -> Self {
        Error { message, location }
    }
}

impl Error {
    pub fn get_line(&self) -> u32 {
        self.location.0
    }

    pub fn get_column(&self) -> u32 {
        self.location.1
    }

    pub fn get_location(&self) -> (u32, u32) {
        self.location
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
