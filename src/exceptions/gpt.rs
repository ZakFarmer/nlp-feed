use std::fmt;

#[derive(Debug)]
pub enum GPTException {
    ResponseTooLongException,
    ResponseContainsProfanity,
    RequestTimedOutException,
}

impl std::error::Error for GPTException {}

impl fmt::Display for GPTException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GPTException::ResponseTooLongException => write!(f, "[GPT] Response was too long."),
            GPTException::ResponseContainsProfanity => {
                write!(f, "[GPT] Response contains profanity.")
            }
            GPTException::RequestTimedOutException => write!(f, "[GPT] Request timed out]"),
        }
    }
}
