mod error;
mod pool;
mod tx;

use crate::pool::{Event, Pool};
use crate::tx::{Transaction, TransactionType};

pub type Result<T> = std::result::Result<T, error::KrctError>;

pub struct Krct {
    pool: Pool,
}

impl Krct {
    pub fn read<P: AsRef<std::path::Path>>(input_file_path: P) -> Result<Self> {
        let mut input_file = Self::load_input_file(input_file_path)?;
        let mut pool = Pool::default();

        for tx in Self::deserialize::<Transaction>(&mut input_file) {
            pool.handle(tx.into())?
        }

        Ok(Krct { pool })
    }

    pub fn dump<W: std::io::Write>(self, writer: W) -> Result<()> {
        let mut writer = csv::Writer::from_writer(writer);
        for client in self.pool.iter() {
            writer.serialize(client)?;
            writer.flush()?;
        }

        Ok(())
    }

    pub fn dump_sorted<W: std::io::Write>(self, writer: W) -> Result<()> {
        let mut writer = csv::Writer::from_writer(writer);
        for client in self.pool.sorted() {
            writer.serialize(client)?;
            writer.flush()?;
        }

        Ok(())
    }

    fn deserialize<'a, T>(file: &'a mut csv::Reader<std::fs::File>) -> impl Iterator<Item = T> + 'a
    where
        T: for<'de> serde::Deserialize<'de> + 'a,
    {
        file.deserialize::<T>().filter_map(std::result::Result::ok)
    }

    fn load_input_file<P>(input_file_path: P) -> csv::Result<csv::Reader<std::fs::File>>
    where
        P: AsRef<std::path::Path>,
    {
        let mut input_file = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_path(input_file_path)?;
        let trimmed_headers = input_file.headers()?.iter().map(str::trim).collect();
        input_file.set_headers(trimmed_headers);

        Ok(input_file)
    }
}
