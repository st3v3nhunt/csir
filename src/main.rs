use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    csir::check_env().with_context(|| "environment was not set up correctly")?;
    env_logger::init();
    csir::run().await.with_context(|| "unable to run csir")?;
    Ok(())
}
