/*
Copyright (c) 2023 Joaquin Maier

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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

