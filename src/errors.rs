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
    pub reason: Option<&'static str>
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

#[derive(Debug, Clone)]
pub struct NotEnoughMoneyError;

impl Error for NotEnoughMoneyError {}

impl std::fmt::Display for NotEnoughMoneyError
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "Not enough money in wallet" )
    }
}

#[derive(Debug, Clone)]
pub struct NonExistentBetError;

impl Error for NonExistentBetError {}

impl std::fmt::Display for NonExistentBetError
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "Attempted to access bet parameter, of value None" )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidStateError;

impl Error for InvalidStateError {}

impl std::fmt::Display for InvalidStateError
{
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> std::fmt::Result {
        write!( f, "Wallet state is invalid" )
    }
}

