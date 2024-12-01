fn main() -> aoc::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    for day in 1..=25 {
        aoc::get_input(2023, day)?;
    }


    Ok(())
}
