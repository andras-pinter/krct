use crate::KrctWorld;
use cucumber::{gherkin::Step, given};
use std::io::Write;

#[given("the following CSV file")]
async fn write_content(w: &mut KrctWorld, step: &Step) {
    w.tempfile
        .as_file()
        .write_all(step.docstring().cloned().unwrap_or_default().as_bytes())
        .expect("Failed to write test file")
}
