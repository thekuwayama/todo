use std::io::{Error, Write};

pub fn add<W: Write>(writer: &mut W, s: &str) -> Result<(), Error> {
    writer.write(format!("[ ] {} ()\n", s).as_bytes())?;
    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut writer = Vec::new();
        assert!(add(&mut writer, "test").is_ok());
        assert_eq!(writer, b"[ ] test ()");
    }
}
