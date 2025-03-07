fn main() -> Result<(), Box<dyn std::error::Error>> {
    postgres_helper::cargo::configure();
    Ok(())
}
