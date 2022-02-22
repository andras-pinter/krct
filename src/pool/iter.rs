use super::{Client, Event, Pool};

impl Pool {
    /// Start shutting down and joining client threads. Returning an iterator, so a client could be
    /// dumped as soon as it finished processing
    pub fn iter(self) -> impl std::iter::Iterator<Item = Client> {
        self.clients.into_iter().map(|(id, (tx, client))| {
            tx.send(Event::Finish).expect("Worker died");
            client
                .join()
                .unwrap_or_else(|err| panic!("Could not join {} worker: {:?}", id, err))
        })
    }

    /// Returns an iterator for the Clients in a sorted form. In this scenario all the client
    /// handlers has to be finished first
    pub fn sorted(self) -> impl std::iter::Iterator<Item = Client> {
        let mut clients = self.iter().collect::<Vec<Client>>();
        clients.sort_by(|first, second| first.id.cmp(&second.id));

        clients.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{Client, Pool};

    #[test]
    fn test_pool_join() {
        let mut clients = std::collections::HashMap::new();
        for id in 1..4 {
            let (tx, rx) = std::sync::mpsc::channel();
            clients.insert(id, (tx, std::thread::spawn(move || Client::new(id, rx))));
        }
        let pool = Pool { clients };

        assert_eq!(pool.iter().collect::<Vec<Client>>().len(), 3);
    }
}
