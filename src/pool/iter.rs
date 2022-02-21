use super::{Client, Event, Pool};

impl Pool {
    pub fn iter(self) -> impl std::iter::Iterator<Item = Client> {
        self.clients.into_iter().map(|(id, (tx, client))| {
            tx.send(Event::Finish).expect("Worker died");
            client
                .join()
                .unwrap_or_else(|err| panic!("Could not join {} worker: {:?}", id, err))
        })
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
