use std::error::Error;

#[derive(Debug, Clone)]
pub struct NotComputedError;

// No source, not required
impl Error for NotComputedError {}

impl std::fmt::Display for NotComputedError
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "Required parameter was None at the time of access." )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidOperationError {
    reason: Option<&'static str>
}

impl InvalidOperationError {
    pub fn new( reason: Option<&'static str> ) -> InvalidOperationError {
        InvalidOperationError { reason }
    }
}

impl Error for InvalidOperationError {}

impl std::fmt::Display for InvalidOperationError
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "Requested operation is invalid. Reason: {:?}.", self.reason )
    }
}
