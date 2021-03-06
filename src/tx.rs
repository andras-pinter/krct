/// Type of the event, could be:
/// * deposit
/// * withdraw
/// * dispute
/// * resolve
/// * chargeback
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,

    #[serde(other)]
    Unknown,
}

/// Each line of teh input CSV file is a well-defined event, described by the `Transaction` struct.
/// All event has a type, described by the `TransactionType` enum. Each event assigned by a client
/// id and a transaction id. Optionally the amount, not all transaction have an amount.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct Transaction {
    #[serde(rename = "type")]
    pub(crate) _type: TransactionType,
    #[serde(rename = "client")]
    pub(crate) client_id: u16,
    #[serde(rename = "tx")]
    pub(crate) transaction_id: u32,
    #[serde(default)]
    pub(crate) amount: Option<f32>,
}

#[cfg(test)]
mod common {
    use std::io::Write;

    pub fn create_test_file(content: &'static str) -> std::fs::File {
        let mut test_file =
            tempfile::NamedTempFile::new().expect("Failed to create temporary file!");
        test_file
            .write_all(content.as_bytes())
            .expect("Failed to write temporary file!");
        test_file.reopen().expect("Failed to open temporary file!")
    }
}

#[cfg(test)]
mod positive_test_cases {
    use super::common::create_test_file;
    use super::{Transaction, TransactionType};
    use crate::Krct;

    #[test]
    fn test_deposit_parsing() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        deposit,1,1,1.0\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_ok(), "{}", record.unwrap_err());
        let record = record.unwrap();
        assert_eq!(record._type, TransactionType::Deposit);
        assert_eq!(record.client_id, 1);
        assert_eq!(record.transaction_id, 1);
        assert_eq!(record.amount, Some(1.0));
    }

    #[test]
    fn test_withdrawal_parsing() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        withdrawal,1,1,1.0\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_ok(), "{}", record.unwrap_err());
        let record = record.unwrap();
        assert_eq!(record._type, TransactionType::Withdrawal);
        assert_eq!(record.client_id, 1);
        assert_eq!(record.transaction_id, 1);
        assert_eq!(record.amount, Some(1.0));
    }

    #[test]
    fn test_dispute_parsing() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        dispute,1,1,\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_ok(), "{}", record.unwrap_err());
        let record = record.unwrap();
        assert_eq!(record._type, TransactionType::Dispute);
        assert_eq!(record.client_id, 1);
        assert_eq!(record.transaction_id, 1);
        assert_eq!(record.amount, None);
    }

    #[test]
    fn test_resolve_parsing() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        resolve,1,1,\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_ok(), "{}", record.unwrap_err());
        let record = record.unwrap();
        assert_eq!(record._type, TransactionType::Resolve);
        assert_eq!(record.client_id, 1);
        assert_eq!(record.transaction_id, 1);
        assert_eq!(record.amount, None);
    }

    #[test]
    fn test_chargeback_parsing() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        chargeback,1,1,\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_ok(), "{}", record.unwrap_err());
        let record = record.unwrap();
        assert_eq!(record._type, TransactionType::Chargeback);
        assert_eq!(record.client_id, 1);
        assert_eq!(record.transaction_id, 1);
        assert_eq!(record.amount, None);
    }

    #[test]
    fn test_parsing_with_whitespaces() {
        let test_case = create_test_file(
            "\
        type,       client, tx, amount\n\
        deposit,    1,      1,  1.0\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_ok(), "{}", record.unwrap_err());
        let record = record.unwrap();
        assert_eq!(record._type, TransactionType::Deposit);
        assert_eq!(record.client_id, 1);
        assert_eq!(record.transaction_id, 1);
        assert_eq!(record.amount, Some(1.0));
    }
}

#[cfg(test)]
mod negative_test_cases {
    use super::common::create_test_file;
    use super::Transaction;
    use crate::{Krct, TransactionType};

    #[test]
    fn test_unknown_transaction_type() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        unknownType,1,1,1.0\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_ok(), "{}", record.unwrap_err());
        let record = record.unwrap();
        assert_eq!(record._type, TransactionType::Unknown);
    }

    #[test]
    fn test_non_numeric_client_id() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        deposit,a,1,1.0\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_err());
    }

    #[test]
    fn test_non_numeric_transaction_id() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        deposit,1,a,1.0\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_err());
    }

    #[test]
    fn test_non_numeric_amount() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        deposit,1,1,a\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_err());
    }

    #[test]
    fn test_missing_transaction_id() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        deposit,1\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_err());
    }

    #[test]
    fn test_missing_transaction_id_and_client_id() {
        let test_case = create_test_file(
            "\
        type,client,tx,amount\n\
        deposit\
        ",
        );
        let mut reader = Krct::get_reader(test_case);
        let mut tx = reader.deserialize::<Transaction>();

        let record = tx.next();
        assert!(record.is_some());
        let record = record.unwrap();
        assert!(record.is_err());
    }
}
