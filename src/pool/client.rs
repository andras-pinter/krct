use crate::pool::Event;
use std::collections::HashMap;
use std::sync::mpsc;

#[derive(Debug, serde::Serialize)]
#[cfg_attr(test, derive(PartialEq))]
pub(in crate::pool) struct Amount<T>(T);

impl std::ops::AddAssign<f32> for Amount<f64> {
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs as f64
    }
}

impl std::ops::SubAssign<f32> for Amount<f64> {
    fn sub_assign(&mut self, rhs: f32) {
        if self.0 >= rhs as f64 {
            self.0 -= rhs as f64
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Client {
    #[serde(rename = "client")]
    pub(in crate::pool) id: u16,
    #[serde(skip_serializing)]
    pub(in crate::pool) transaction_history: HashMap<u32, Amount<f32>>,
    pub(in crate::pool) available: Amount<f64>,
    pub(in crate::pool) held: Amount<f64>,
    pub(in crate::pool) total: Amount<f64>,
    pub(in crate::pool) locked: bool,

    #[serde(skip_serializing)]
    channel: mpsc::Receiver<Event>,
}

impl Client {
    pub fn new(client_id: u16, channel: mpsc::Receiver<Event>) -> Self {
        Self {
            id: client_id,
            channel,

            transaction_history: HashMap::new(),
            available: Amount(0.0),
            held: Amount(0.0),
            total: Amount(0.0),
            locked: false,
        }
    }

    pub fn start_handling(mut self) -> Self {
        while let Ok(event) = self.channel.recv() {
            match event {
                Event::Finish => break,
                Event::Deposit { amount, tx, .. } => {
                    self.transaction_history.insert(tx, Amount(amount));
                    self.available += amount;
                    self.total += amount;
                }
                Event::Withdrawal { amount, .. } => {
                    self.available -= amount;
                    self.total -= amount;
                }
                _ => unimplemented!(),
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
    use super::{Amount, Client};
    use std::io::Read;

    #[test]
    fn test_client_serialization() {
        let (_, rx) = std::sync::mpsc::channel();
        let mut tempfile = tempfile::NamedTempFile::new().expect("Failed to create testfile");
        let mut transaction_history = std::collections::HashMap::new();
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
