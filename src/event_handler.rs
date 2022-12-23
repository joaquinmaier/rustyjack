use std::sync::mpsc;

pub enum Event
{
    ExitRequest,
    PlayHit( usize ),
    PlayStand( usize )
}

pub struct EventHandler
{
    events: Vec<Event>,
    msg_sender: mpsc::Sender;
}

impl EventHandler
{
    pub fn new() -> ( EventHandler, mpsc::Receiver ) {
        let (sender, receiver) = mpsc::channel();
        return ( EventHandler { events: Vec::new(), msg_sender: sender }, receiver );
    }

    pub fn push( &mut self, event: Event ) {
        self.events.push( event );
    }

    pub fn process( &mut self ) {
        for event in self.events.iter() {
            match event {
                Event::ExitRequest => {  },
                Event::PlayHit( index ) => {  },
                Event::PlayStand( index ) => {  }
            }
        }
    }
}

