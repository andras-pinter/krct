use super::amount::Amount;
use std::collections::HashMap;

/// State of a transaction history to indicate if a transaction is
/// * Recorded: base state
/// * Held: the corresponding transaction is under dispute
/// * ChargedBack: the corresponding transaction is changed back
#[derive(PartialEq, Debug)]
pub(in crate::pool) enum State {
    Recorded,
    Held,
    ChargedBack,
}

/// Incoming transaction history to record all incoming amounts to be able to dispute a previous
/// transaction
#[derive(Default, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub(in crate::pool) struct History<K, V>(HashMap<K, (Amount<V>, State)>)
where
    K: Eq + std::hash::Hash;

impl<K, V> History<K, V>
where
    K: Eq + std::hash::Hash,
{
    /// Add an incoming transaction to history
    pub(in crate::pool) fn insert(&mut self, id: K, amount: Amount<V>) {
        self.0.insert(id, (amount, State::Recorded));
    }

    /// Select and get an incoming transaction from history with the given state
    pub(in crate::pool) fn select(&self, id: K, state: State) -> Option<&Amount<V>> {
        match self.0.get(&id) {
            Some(transaction) if transaction.1 == state => Some(&transaction.0),
            _ => None,
        }
    }

    /// Set the state of an incoming transaction in the history
    pub(in crate::pool) fn set_state(&mut self, id: K, state: State) {
        if let Some(transaction) = self.0.get_mut(&id) {
            transaction.1 = state;
        }
    }
}

#[cfg(test)]
impl<K, V, const N: usize> From<[(K, V, State); N]> for History<K, V>
where
    K: Eq + std::hash::Hash + std::default::Default + Clone,
    V: std::default::Default,
{
    fn from(data: [(K, V, State); N]) -> Self {
        let mut history = History::default();
        for (key, value, state) in data {
            history.insert(key.clone(), Amount(value));
            history.set_state(key, state);
        }
        history
    }
}

#[cfg(test)]
mod tests {
    use super::{Amount, History, State};

    #[test]
    fn test_selecting_from_history_with_hit() {
        let mut history = History::default();
        history.insert(1, Amount(1.0));
        assert!(history.select(1, State::Recorded).is_some());
    }

    #[test]
    fn test_selecting_from_history_with_no_hit() {
        let mut history = History::default();
        history.insert(1, Amount(1.0));
        history.set_state(1, State::Held);
        assert!(history.select(1, State::Recorded).is_none());
    }
}
