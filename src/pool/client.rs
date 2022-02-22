use super::amount::Amount;
use super::history::{History, State};
use crate::pool::Event;
use std::sync::mpsc;

/// Main business logic, handling events corresponding to the given client.
#[derive(Debug, serde::Serialize)]
pub struct Client {
    #[serde(rename = "client")]
    pub(in crate::pool) id: u16,
    #[serde(skip_serializing)]
    pub(in crate::pool) transaction_history: History<u32, f32>,
    pub(in crate::pool) available: Amount<f64>,
    pub(in crate::pool) held: Amount<f64>,
    pub(in crate::pool) total: Amount<f64>,
    pub(in crate::pool) locked: bool,

    #[serde(skip_serializing)]
    channel: mpsc::Receiver<Event>,
}

impl Client {
    /// Constructing a new client with the given Client ID and the receiver part of the
    /// communication channel
    pub fn new(client_id: u16, channel: mpsc::Receiver<Event>) -> Self {
        Self {
            id: client_id,
            channel,

            transaction_history: History::default(),
            available: Amount(0.0),
            held: Amount(0.0),
            total: Amount(0.0),
            locked: false,
        }
    }

    /// Event handling thread, receiving events from the sender via the previously given channel.
    ///
    /// # Events
    /// * Deposit: increase the available and total amount
    /// * Withdrawal: decreasing the available and total amount
    /// * Dispute: decreasing the available, but not the talal. Also, increasing the held amount
    /// * Resolve: a previously disputed transaction resolved and the available amount should be increased
    /// * Chargeback: a previously disputed transaction should be charged back. The total and the available
    ///   amount should be decreased and the client has to be locked.
    ///
    /// # Finish
    /// Special event to indicate the processing of the events should be finished and the handling
    /// thread has to be stopped
    pub fn start_handling(mut self) -> Self {
        while let Ok(event) = self.channel.recv() {
            match event {
                Event::Finish => break,
                Event::Deposit { amount, tx, .. } if !self.locked => {
                    self.transaction_history.insert(tx, Amount(amount));
                    self.available += amount;
                    self.total += amount;
                }
                Event::Withdrawal { amount, .. } if !self.locked && self.available >= amount => {
                    self.available -= amount;
                    self.total -= amount;
                }
                Event::Dispute { tx, .. } if !self.locked => {
                    if let Some(amount) = self.transaction_history.select(tx, State::Recorded) {
                        self.available -= amount;
                        self.held += amount;
                        self.transaction_history.set_state(tx, State::Held);
                    }
                }
                Event::Resolve { tx, .. } if !self.locked => {
                    if let Some(amount) = self.transaction_history.select(tx, State::Held) {
                        self.held -= amount;
                        self.available += amount;
                        self.transaction_history.set_state(tx, State::Recorded);
                    }
                }
                Event::Chargeback { tx, .. } if !self.locked => {
                    if let Some(amount) = self.transaction_history.select(tx, State::Held) {
                        self.held -= amount;
                        self.total -= amount;
                        self.locked = true;
                        self.transaction_history.set_state(tx, State::ChargedBack);
                    }
                }
                _ => (),
            }
        }

        self
    }
}

#[cfg(test)]
impl From<f32> for Amount<f32> {
    fn from(val: f32) -> Self {
        Amount(val)
    }
}

#[cfg(test)]
impl From<f64> for Amount<f64> {
    fn from(val: f64) -> Self {
        Amount(val)
    }
}

#[cfg(test)]
mod tests {
    use super::{Amount, Client, History};
    use std::io::Read;

    #[test]
    fn test_client_serialization() {
        let (_, rx) = std::sync::mpsc::channel();
        let mut tempfile = tempfile::NamedTempFile::new().expect("Failed to create testfile");
        let mut transaction_history = History::default();
        transaction_history.insert(1, Amount(1.0));
        let client = Client {
            id: 1,
            transaction_history,
            available: Amount(10.0),
            held: Amount(2.0),
            total: Amount(12.0),
            locked: false,
            channel: rx,
        };
        let writer = csv::Writer::from_path(tempfile.as_ref());
        assert!(writer.is_ok(), "{}", writer.unwrap_err());
        let mut writer = writer.unwrap();

        assert!(writer.serialize(client).is_ok());
        assert!(writer.flush().is_ok());

        let mut buffer = String::new();
        tempfile
            .read_to_string(&mut buffer)
            .expect("Failed to read testfile");
        assert_eq!(
            buffer,
            "client,available,held,total,locked\n\
            1,10.0,2.0,12.0,false\n"
        )
    }
}
