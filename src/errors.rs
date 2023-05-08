#[derive(Debug)]
pub enum AppErrors {
    ErrorAtYourTestCase,
    ErrorToReadOutputFile,
    NotHasErrors,
}

impl AppErrors {
    pub fn to_message(&self) -> &str {
        match self {
            AppErrors::ErrorAtYourTestCase => "Error at your test case",
            AppErrors::ErrorToReadOutputFile => "Error to read output file",
            AppErrors::NotHasErrors => "Finish, no errors to insert...",
        }
    }
}
