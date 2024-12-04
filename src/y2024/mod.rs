pub mod d01;
pub mod d02;
pub mod d03;

pub fn main() -> crate::Result<()> {
    d01::main()?;
    d02::main()?;
    d03::main()?;

    Ok(())
}
