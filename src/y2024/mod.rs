pub mod d01;
pub mod d02;

pub fn main() -> crate::Result<()> {
    d01::main()?;
    d02::main()?;

    Ok(())
}