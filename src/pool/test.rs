use super::Pool;
use crate::pool::client::Amount;
use crate::Event;
use std::collections::HashMap;

fn send(pool: &mut Pool, event: Event) {
    assert!(pool.handle(event).is_ok())
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
    let mut result = HashMap::new();
    for c in pool.iter() {
        result.insert(c.id, c);
    }

    let client_1 = result.get(&1);
    assert!(client_1.is_some());
    let client_1 = client_1.unwrap();
    let mut expected_history = HashMap::new();
    expected_history.insert(1, Amount::from(1.0));
    expected_history.insert(2, Amount::from(2.0));
    assert_eq!(client_1.transaction_history, expected_history);
    assert_eq!(client_1.available, Amount::from(3.0));
    assert_eq!(client_1.held, Amount::from(0.0));
    assert_eq!(client_1.total, Amount::from(3.0));
    assert_eq!(client_1.locked, false);

    let client_2 = result.get(&2);
    assert!(client_2.is_some());
    let client_2 = client_2.unwrap();
    let mut expected_history = HashMap::new();
    expected_history.insert(3, Amount::from(1.0));
    assert_eq!(client_2.transaction_history, expected_history);
    assert_eq!(client_2.available, Amount::from(1.0));
    assert_eq!(client_2.held, Amount::from(0.0));
    assert_eq!(client_2.total, Amount::from(1.0));
    assert_eq!(client_2.locked, false);
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
    let mut result = HashMap::new();
    for c in pool.iter() {
        result.insert(c.id, c);
    }

    let client_1 = result.get(&1);
    assert!(client_1.is_some());
    let client_1 = client_1.unwrap();
    let mut expected_history = HashMap::new();
    expected_history.insert(1, Amount::from(2.0));
    assert_eq!(client_1.transaction_history, expected_history);
    assert_eq!(client_1.available, Amount::from(1.0));
    assert_eq!(client_1.held, Amount::from(0.0));
    assert_eq!(client_1.total, Amount::from(1.0));
    assert_eq!(client_1.locked, false);

    let client_2 = result.get(&2);
    assert!(client_2.is_some());
    let client_2 = client_2.unwrap();
    let expected_history = HashMap::new();
    assert_eq!(client_2.transaction_history, expected_history);
    assert_eq!(client_2.available, Amount::from(0.0));
    assert_eq!(client_2.held, Amount::from(0.0));
    assert_eq!(client_2.total, Amount::from(0.0));
    assert_eq!(client_2.locked, false);
}
