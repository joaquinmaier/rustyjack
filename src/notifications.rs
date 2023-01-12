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

