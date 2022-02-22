use super::amount::Amount;
use super::client::Client;
use super::history::{History, State};
use super::Pool;
use crate::Event;
use std::collections::HashMap;

struct ClientAssertion {
    id: u16,
    transaction_history: History<u32, f32>,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

fn send(pool: &mut Pool, event: Event) {
    assert!(pool.handle(event).is_ok())
}

fn assert_clients(pool: Pool, expected: Vec<ClientAssertion>) {
    assert_eq!(
        pool.len(),
        expected.len(),
        "Pool length and Assertion asset length should be equal"
    );
    let result_set = pool
        .iter()
        .map(|client| (client.id, client))
        .collect::<HashMap<u16, Client>>();

    for expected_client in expected {
        let client = result_set.get(&expected_client.id);
        assert!(
            client.is_some(),
            "Client should be found with id: {}",
            expected_client.id
        );
        let client = client.unwrap();
        assert_eq!(
            client.transaction_history,
            expected_client.transaction_history
        );
        assert_eq!(client.available, Amount::from(expected_client.available));
        assert_eq!(client.held, Amount::from(expected_client.held));
        assert_eq!(client.total, Amount::from(expected_client.total));
        assert_eq!(client.locked, expected_client.locked);
    }
}

#[test]
fn test_deposit_flow() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 1.0,
        },
    );
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 2,
            amount: 2.0,
        },
    );
    send(
        &mut pool,
        Event::Deposit {
            client: 2,
            tx: 3,
            amount: 1.0,
        },
    );
    assert_clients(
        pool,
        vec![
            ClientAssertion {
                id: 1,
                available: 3.0,
                held: 0.0,
                total: 3.0,
                locked: false,
                transaction_history: History::from([
                    (1, 1.0, State::Recorded),
                    (2, 2.0, State::Recorded),
                ]),
            },
            ClientAssertion {
                id: 2,
                available: 1.0,
                held: 0.0,
                total: 1.0,
                locked: false,
                transaction_history: History::from([(3, 1.0, State::Recorded)]),
            },
        ],
    );
}

#[test]
fn test_withdrawal_flow() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 2.0,
        },
    );
    send(
        &mut pool,
        Event::Withdrawal {
            client: 1,
            tx: 2,
            amount: 1.0,
        },
    );
    send(
        &mut pool,
        Event::Withdrawal {
            client: 2,
            tx: 3,
            amount: 1.0,
        },
    );
    assert_clients(
        pool,
        vec![
            ClientAssertion {
                id: 1,
                available: 1.0,
                held: 0.0,
                total: 1.0,
                locked: false,
                transaction_history: History::from([(1, 2.0, State::Recorded)]),
            },
            ClientAssertion {
                id: 2,
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: false,
                transaction_history: History::default(),
            },
        ],
    );
}

#[test]
fn test_dispute_flow() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 2.0,
        },
    );
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 2,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Dispute { client: 1, tx: 2 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 2.0,
            held: 1.0,
            total: 3.0,
            locked: false,
            transaction_history: History::from([(1, 2.0, State::Recorded), (2, 1.0, State::Held)]),
        }],
    );
}

#[test]
fn test_resolve_flow() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 2.0,
        },
    );
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 2,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Dispute { client: 1, tx: 2 });
    send(&mut pool, Event::Resolve { client: 1, tx: 2 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 3.0,
            held: 0.0,
            total: 3.0,
            locked: false,
            transaction_history: History::from([
                (1, 2.0, State::Recorded),
                (2, 1.0, State::Recorded),
            ]),
        }],
    );
}

#[test]
fn test_chargeback_flow() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 2.0,
        },
    );
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 2,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Dispute { client: 1, tx: 2 });
    send(&mut pool, Event::Chargeback { client: 1, tx: 2 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 2.0,
            held: 0.0,
            total: 2.0,
            locked: true,
            transaction_history: History::from([
                (1, 2.0, State::Recorded),
                (2, 1.0, State::ChargedBack),
            ]),
        }],
    );
}

#[test]
fn test_deposit_after_chargeback() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 2.0,
        },
    );
    send(&mut pool, Event::Dispute { client: 1, tx: 1 });
    send(&mut pool, Event::Chargeback { client: 1, tx: 1 });
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 2,
            amount: 1.0,
        },
    );
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: true,
            transaction_history: History::from([(1, 2.0, State::ChargedBack)]),
        }],
    );
}

#[test]
fn test_dispute_event_ignored_upon_non_existent_transaction_id() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Dispute { client: 1, tx: 2 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 1.0,
            held: 0.0,
            total: 1.0,
            locked: false,
            transaction_history: History::from([(1, 1.0, State::Recorded)]),
        }],
    );
}

#[test]
fn test_resolve_event_ignored_upon_non_existent_transaction_id() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Dispute { client: 1, tx: 1 });
    send(&mut pool, Event::Resolve { client: 1, tx: 2 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 0.0,
            held: 1.0,
            total: 1.0,
            locked: false,
            transaction_history: History::from([(1, 1.0, State::Held)]),
        }],
    );
}

#[test]
fn test_resolve_event_ignored_upon_non_disputed_transaction_id() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Resolve { client: 1, tx: 1 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 1.0,
            held: 0.0,
            total: 1.0,
            locked: false,
            transaction_history: History::from([(1, 1.0, State::Recorded)]),
        }],
    );
}

#[test]
fn test_chargeback_event_ignored_upon_non_existent_transaction_id() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Dispute { client: 1, tx: 1 });
    send(&mut pool, Event::Chargeback { client: 1, tx: 2 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 0.0,
            held: 1.0,
            total: 1.0,
            locked: false,
            transaction_history: History::from([(1, 1.0, State::Held)]),
        }],
    );
}

#[test]
fn test_chargeback_event_ignored_upon_non_disputed_transaction_id() {
    let mut pool = Pool::default();
    send(
        &mut pool,
        Event::Deposit {
            client: 1,
            tx: 1,
            amount: 1.0,
        },
    );
    send(&mut pool, Event::Chargeback { client: 1, tx: 1 });
    assert_clients(
        pool,
        vec![ClientAssertion {
            id: 1,
            available: 1.0,
            held: 0.0,
            total: 1.0,
            locked: false,
            transaction_history: History::from([(1, 1.0, State::Recorded)]),
        }],
    );
}
