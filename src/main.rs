use anyhow::Context;
use csir::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await.with_context(|| "unable to run csir")?;
    Ok(())
}
