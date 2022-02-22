use crate::KrctWorld;
use cucumber::{gherkin::Step, then};

#[then("the following output should be generated")]
async fn assert_content(w: &mut KrctWorld, step: &Step) {
    pretty_assertions::assert_eq!(
        w.output.buffer.trim(),
        step.docstring().cloned().unwrap_or_default().trim()
    )
}
