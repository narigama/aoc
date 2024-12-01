fn main() -> aoc::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    aoc::y2024::d01::main()?;

    Ok(())
}
