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

