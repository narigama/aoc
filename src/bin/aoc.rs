fn main() -> aoc::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    aoc::y2024::main()?;

    Ok(())
}
