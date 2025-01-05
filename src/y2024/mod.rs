pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;

pub fn main() -> crate::Result<()> {
    d01::main()?;
    d02::main()?;
    d03::main()?;
    d04::main()?;
    d05::main()?;
    d06::main()?;
    d07::main()?;
    d08::main()?;

    Ok(())
}
