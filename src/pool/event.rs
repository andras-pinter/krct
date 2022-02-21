use crate::{Transaction, TransactionType};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Event {
    Deposit { client: u16, tx: u32, amount: f32 },
    Withdrawal { client: u16, tx: u32, amount: f32 },
    Dispute { client: u16, tx: u32 },
    Resolve { client: u16, tx: u32 },
    Chargeback { client: u16, tx: u32 },

    Finish,
}

impl From<Transaction> for Event {
    fn from(tx: Transaction) -> Self {
        match tx._type {
            TransactionType::Deposit => Event::Deposit {
                client: tx.client_id,
                tx: tx.transaction_id,
                amount: tx.amount.unwrap_or_default(),
            },
            TransactionType::Withdrawal => Event::Withdrawal {
                client: tx.client_id,
                tx: tx.transaction_id,
                amount: tx.amount.unwrap_or_default(),
            },
            TransactionType::Dispute => Event::Dispute {
                client: tx.client_id,
                tx: tx.transaction_id,
            },
            TransactionType::Resolve => Event::Resolve {
                client: tx.client_id,
                tx: tx.transaction_id,
            },
            TransactionType::Chargeback => Event::Chargeback {
                client: tx.client_id,
                tx: tx.transaction_id,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Transaction, TransactionType};

    #[test]
    fn test_deposit_event() {
        let tx = Transaction {
            _type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 1,
            amount: Some(2.0),
        };

        assert_eq!(
            Event::from(tx),
            Event::Deposit {
                client: 1,
                tx: 1,
                amount: 2.0
            }
        );
    }

    #[test]
    fn test_deposit_event_without_amount() {
        let tx = Transaction {
            _type: TransactionType::Deposit,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        };

        assert_eq!(
            Event::from(tx),
            Event::Deposit {
                client: 1,
                tx: 1,
                amount: 0.0
            }
        );
    }

    #[test]
    fn test_withdrawal_event() {
        let tx = Transaction {
            _type: TransactionType::Withdrawal,
            client_id: 1,
            transaction_id: 1,
            amount: Some(2.0),
        };

        assert_eq!(
            Event::from(tx),
            Event::Withdrawal {
                client: 1,
                tx: 1,
                amount: 2.0
            }
        );
    }

    #[test]
    fn test_withdrawal_event_without_amount() {
        let tx = Transaction {
            _type: TransactionType::Withdrawal,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        };

        assert_eq!(
            Event::from(tx),
            Event::Withdrawal {
                client: 1,
                tx: 1,
                amount: 0.0
            }
        );
    }

    #[test]
    fn test_dispute_event() {
        let tx = Transaction {
            _type: TransactionType::Dispute,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        };

        assert_eq!(Event::from(tx), Event::Dispute { client: 1, tx: 1 });
    }

    #[test]
    fn test_dispute_event_amount_is_ignored() {
        let tx = Transaction {
            _type: TransactionType::Dispute,
            client_id: 1,
            transaction_id: 1,
            amount: Some(2.0),
        };

        assert_eq!(Event::from(tx), Event::Dispute { client: 1, tx: 1 });
    }

    #[test]
    fn test_resolve_event() {
        let tx = Transaction {
            _type: TransactionType::Resolve,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        };

        assert_eq!(Event::from(tx), Event::Resolve { client: 1, tx: 1 });
    }

    #[test]
    fn test_resolve_event_amount_is_ignored() {
        let tx = Transaction {
            _type: TransactionType::Resolve,
            client_id: 1,
            transaction_id: 1,
            amount: Some(2.0),
        };

        assert_eq!(Event::from(tx), Event::Resolve { client: 1, tx: 1 });
    }

    #[test]
    fn test_chargeback_event() {
        let tx = Transaction {
            _type: TransactionType::Chargeback,
            client_id: 1,
            transaction_id: 1,
            amount: None,
        };

        assert_eq!(Event::from(tx), Event::Chargeback { client: 1, tx: 1 });
    }

    #[test]
    fn test_chargeback_event_amount_is_ignored() {
        let tx = Transaction {
            _type: TransactionType::Chargeback,
            client_id: 1,
            transaction_id: 1,
            amount: Some(2.0),
        };

        assert_eq!(Event::from(tx), Event::Chargeback { client: 1, tx: 1 });
    }
}