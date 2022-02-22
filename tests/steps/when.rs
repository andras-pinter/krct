use crate::KrctWorld;
use cucumber::when;

#[when("the engine is executed")]
async fn write_content(w: &mut KrctWorld) {
    krct::Krct::try_from(w.tempfile.path())
        .expect("Error occurred running the engine!")
        .dump_sorted(&mut w.output)
        .expect("Failed to write output");
}
