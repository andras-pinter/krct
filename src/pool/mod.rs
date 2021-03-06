mod amount;
mod client;
mod event;
mod history;
mod iter;
#[cfg(test)]
mod test;

use crate::error::KrctError;
use crate::pool::client::Client;
pub use event::Event;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread::JoinHandle;

/// Pre-allocating space for N clients
const CLIENT_PREALLOCATE: usize = 20;

pub struct Pool {
    clients: HashMap<u16, (mpsc::Sender<Event>, JoinHandle<Client>)>,
}

impl Default for Pool {
    fn default() -> Self {
        Self {
            clients: HashMap::with_capacity(CLIENT_PREALLOCATE),
        }
    }
}

impl Pool {
    /// The client pool is responsible handling clients and dispatches the events to the
    /// corresponding client.
    ///
    /// # Error
    /// If an event arrives, which cannot be handled by the client.
    pub fn handle(&mut self, event: Event) -> crate::Result<()> {
        let client = match event {
            Event::Deposit { client, .. } => self.get_or_insert(client),
            Event::Withdrawal { client, .. } => self.get_or_insert(client),
            Event::Dispute { client, .. } => self.get_or_insert(client),
            Event::Resolve { client, .. } => self.get_or_insert(client),
            Event::Chargeback { client, .. } => self.get_or_insert(client),
            _ => return Ok(()),
        };

        client.0.send(event).map_err(KrctError::Handler)
    }

    /// Get a client or initialize a new one, if a previously not known Client ID arrives
    fn get_or_insert(&mut self, client_id: u16) -> &mut (mpsc::Sender<Event>, JoinHandle<Client>) {
        self.clients.entry(client_id).or_insert_with(|| {
            let (tx, rx) = mpsc::channel::<Event>();
            let client = Client::new(client_id, rx);

            (tx, std::thread::spawn(move || client.start_handling()))
        })
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.clients.len()
    }
}
