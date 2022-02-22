mod steps;

use cucumber::WorldInit;

#[derive(Debug, Default)]
struct Output {
    buffer: String,
}

impl std::io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let part = String::from_utf8_lossy(buf);
        self.buffer.push_str(&part);

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, cucumber::WorldInit)]
struct KrctWorld {
    tempfile: tempfile::NamedTempFile,
    output: Output,
}

#[async_trait::async_trait(?Send)]
impl cucumber::World for KrctWorld {
    type Error = std::convert::Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            tempfile: tempfile::NamedTempFile::new()
                .expect("Failed to initialize test environment"),
            output: Output::default(),
        })
    }
}

#[tokio::main]
async fn main() {
    KrctWorld::run("features/").await;
}
