mod error;
mod pool;
mod tx;

use crate::pool::{Event, Pool};
use crate::tx::{Transaction, TransactionType};

pub type Result<T> = std::result::Result<T, error::KrctError>;

pub struct Krct {
    pool: Pool,
}

impl TryFrom<std::path::PathBuf> for Krct {
    type Error = error::KrctError;

    fn try_from(input_file_path: std::path::PathBuf) -> std::result::Result<Self, Self::Error> {
        let input_file = std::fs::File::open(input_file_path)?;
        Self::read(input_file)
    }
}

impl TryFrom<&std::path::Path> for Krct {
    type Error = error::KrctError;

    fn try_from(value: &std::path::Path) -> std::result::Result<Self, Self::Error> {
        Krct::try_from(value.to_path_buf())
    }
}

impl Krct {
    /// Reads the given input CSV steam and reads it line by line. Each line is a well defined
    /// event belongs to a client. Each event processed by the corresponding client thread.
    pub fn read<R: std::io::Read>(reader: R) -> Result<Self> {
        let mut reader = Self::get_reader(reader);
        let mut pool = Pool::default();

        for tx in Self::deserialize::<Transaction, R>(&mut reader) {
            pool.handle(tx.into())?
        }

        Ok(Krct { pool })
    }

    /// When all events are finished processing, the result dumped to the given writer.
    pub fn dump<W: std::io::Write>(self, writer: W) -> Result<()> {
        let mut writer = csv::Writer::from_writer(writer);
        for client in self.pool.iter() {
            writer.serialize(client)?;
            writer.flush()?;
        }

        Ok(())
    }

    /// Dumps the result set sorted by the client identifier
    pub fn dump_sorted<W: std::io::Write>(self, writer: W) -> Result<()> {
        let mut writer = csv::Writer::from_writer(writer);
        for client in self.pool.sorted() {
            writer.serialize(client)?;
            writer.flush()?;
        }

        Ok(())
    }

    fn deserialize<'a, T, R>(reader: &'a mut csv::Reader<R>) -> impl Iterator<Item = T> + 'a
    where
        T: for<'de> serde::Deserialize<'de> + 'a,
        R: std::io::Read,
    {
        reader
            .deserialize::<T>()
            .filter_map(std::result::Result::ok)
    }

    fn get_reader<R: std::io::Read>(reader: R) -> csv::Reader<R> {
        csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(reader)
    }
}
