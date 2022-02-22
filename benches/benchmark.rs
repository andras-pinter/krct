use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::Rng;

const SAMPLE: usize = 10;

const TRANSACTIONS: usize = 10_000_000;
const CLIENTS: usize = 10;
const MAX_AMOUNT: f32 = 100.0;
const TX_TYPES: [&'static str; 5] = ["deposit", "withdrawal", "dispute", "resolve", "chargeback"];

#[derive(Default)]
struct Sink;

impl std::io::Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct CsvInput {
    current_line: usize,
    buffer: Vec<u8>,
    tx_id: usize,

    _rng: rand::rngs::ThreadRng,
}

impl Default for CsvInput {
    fn default() -> Self {
        CsvInput {
            current_line: TRANSACTIONS,
            buffer: Vec::new(),
            tx_id: 0usize,
            _rng: rand::thread_rng(),
        }
    }
}

impl CsvInput {
    fn header() -> &'static str {
        "type,client,tx,amount\n"
    }

    fn line(&mut self) -> String {
        self.current_line -= 1;
        let transaction_type = *TX_TYPES
            .choose(&mut self._rng)
            .expect("Empty Transaction type asset");
        format!(
            "{transaction_type},{client_id},{transaction_id},{amount}\n",
            transaction_type = transaction_type,
            client_id = self._rng.gen_range(1..CLIENTS + 1),
            transaction_id = match transaction_type {
                "deposit" | "withdrawal" => {
                    self.tx_id += 1;
                    self.tx_id
                }
                _ if self.tx_id > 1 => self._rng.gen_range(1..self.tx_id),
                _ => 1,
            },
            amount = match transaction_type {
                "deposit" | "withdrawal" => self._rng.gen_range(0.0..MAX_AMOUNT).to_string(),
                _ => "".to_string(),
            }
        )
    }
}

impl std::io::Read for CsvInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        while buf.len() > self.buffer.len() {
            match self.current_line {
                0 => break,
                TRANSACTIONS => {
                    self.buffer.extend_from_slice(Self::header().as_bytes());
                    let line = self.line();
                    self.buffer.extend_from_slice(line.as_bytes());
                }
                _ => {
                    let line = self.line();
                    self.buffer.extend_from_slice(line.as_bytes())
                }
            }
        }

        match self.current_line {
            0 => {
                let len = self.buffer.len();
                buf[..len].copy_from_slice(&self.buffer);
                self.buffer.clear();

                Ok(len)
            }
            _ => {
                buf.copy_from_slice(&self.buffer[..buf.len()]);
                self.buffer = (&self.buffer[buf.len()..]).to_vec();

                Ok(buf.len())
            }
        }
    }
}

pub fn krct_benchmark(c: &mut Criterion) {
    c.bench_function(&format!("Bench with {} lines", TRANSACTIONS), |bencher| {
        bencher.iter(|| {
            let input = CsvInput::default();
            let output = Sink::default();
            krct::Krct::read(input)
                .expect("Error occurred meanwhile benching Krct")
                .dump(output)
                .expect("Error occurred meanwhile benching Krct");
        })
    });
}

criterion_group! {
    name = krct;
    config = criterion::Criterion::default()
        .measurement_time(std::time::Duration::from_secs(200))
        .sample_size(SAMPLE);
    targets = krct_benchmark
}
criterion_main!(krct);
