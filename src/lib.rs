pub mod util;
pub mod y2024;

use std::{io::Write, str::FromStr, sync::Arc};

use eyre::OptionExt;

pub type Error = eyre::ErrReport;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pair<T> {
    left: T,
    right: T,
}

pub const BASE_URL: &str = "https://adventofcode.com";

pub fn get_input(year: u64, day: u64) -> Result<String> {
    // validate input
    eyre::ensure!((2015..=2024).contains(&year), "aoc years must be between 2015 and 2024");
    eyre::ensure!((1..=25).contains(&day), "aoc days must be between 01 and 25");

    let input_path = std::path::PathBuf::new().join("input").join(format!("{year}")).join(format!("{day:0>2}.txt"));
    if !input_path.is_file() {
        tracing::info!("fetching y{year}d{day:0>2} from the internet");

        // grab the AOC_TOKEN
        let aoc_token = std::env::var("AOC_TOKEN")?;

        // build a cookie jar
        let jar = Arc::new(reqwest::cookie::Jar::default());
        jar.add_cookie_str(&format!("session={aoc_token}"), &reqwest::Url::from_str(BASE_URL)?);

        // fetch the input from the site
        let input = reqwest::blocking::ClientBuilder::new()
            .cookie_provider(jar)
            .build()?
            .get(format!("{BASE_URL}/{year}/day/{day}/input"))
            .send()?
            .error_for_status()?
            .text()?;

        // ensure the parent directory structure exists
        std::fs::create_dir_all(input_path.parent().ok_or_eyre("unable to get parent dir of target path")?)?;

        // write the contents to the file
        std::fs::File::create(&input_path)?.write_all(input.as_bytes())?;
    }

    Ok(std::fs::read_to_string(&input_path)?)
}
