pub mod errors;
pub mod types;

pub fn get_home() -> Result<String, Box<dyn std::error::Error>> {
    let home_path = std::env::var("HOME")?;
    Ok(home_path)
}
