#[derive(thiserror::Error, Debug)]
pub enum KrctError {
    #[error("Input file error: {0}")]
    CsvFileRead(#[from] csv::Error),
    #[error("Dump error: {0}")]
    CsvDump(#[from] std::io::Error),
    #[error("Error handling even: {0}")]
    Handler(#[from] std::sync::mpsc::SendError<crate::Event>),
    #[error("Unknown even: {0:?}")]
    UnkUnknownEvent(super::Event),
}
