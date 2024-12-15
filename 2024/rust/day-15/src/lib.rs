use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse input: {0}")]
    #[diagnostic(code(aoc::parse_error))]
    ParseError(String),
}

// Implement conversion from nom errors to custom error type
impl<T> From<nom::error::Error<T>> for AocError 
where
    T: std::fmt::Display,
{
    fn from(err: nom::error::Error<T>) -> Self {
        AocError::ParseError(format!("{}", err))
    }
}

// Helper for converting nom's IResult to Result<T, AocError>
impl AocError {
    pub fn from_nom_error<I, T>(input: I, err: nom::Err<nom::error::Error<I>>) -> Self 
    where
        I: std::fmt::Display,
    {
        match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                AocError::ParseError(format!("at `{}`: {:?}", e.input, e.code))
            }
            nom::Err::Incomplete(_) => {
                AocError::ParseError("incomplete input".to_string())
            }
        }
    }
}

// // Convenience trait for converting IResult to Result
// pub trait IntoAocResult<T> {
//     fn finish(self, input: &str) -> Result<T, AocError>;
// }

// impl<I, T> IntoAocResult<T> for nom::IResult<I, T>
// where
//     I: std::fmt::Display,
// {
//     fn finish(self, input: &str) -> Result<T, AocError> {
//         self.map(|(_, data)| data)
//             .map_err(|e| AocError::from_nom_error(input, e))
//     }
// }

pub mod part1;
pub mod part2;