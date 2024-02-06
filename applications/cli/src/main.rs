use crate::cli::Cli;

mod cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::init();

    println!("{:?}", cli);
    Ok(())
}
