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

use std::collections::VecDeque;
use colour::*;

pub enum NotificationType
{
    ERROR,
    INFO
}

pub struct Notification
{
    pub notification_type: NotificationType,
    pub message: String
}

impl Notification
{
    pub fn new( notification_type: NotificationType, message: String ) -> Notification {
        Notification { notification_type, message }
    }

    pub fn print( &self ) {
        match self.notification_type {
            NotificationType::ERROR     => {
                dark_red_ln!( "[ERROR] {}", self.message );
            },
            NotificationType::INFO      => {
                cyan_ln!( "[INFO] {}", self.message );
            }
        }
    }
}

pub struct NotificationBuffer
{
    buffer: VecDeque<Notification>
}

impl NotificationBuffer
{
    pub fn new() -> NotificationBuffer {
        NotificationBuffer { buffer: VecDeque::new() }
    }

    pub fn print_all( &mut self ) {
        if self.buffer.len() == 0   { return; }

        print!( "\n" );

        for notification in self.buffer.iter_mut() {
            notification.print();
        }

        self.buffer.clear();
    }

    pub fn add( &mut self, new_notification: Notification ) {
        self.buffer.push_back( new_notification );
    }
}

