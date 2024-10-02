use std::{fs::File, io::Read};

use anyhow::Ok;
use base64::prelude::*;

use crate::cli::Base64Format;

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf);
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{cli::Base64Format, process_decode, process_encode};

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
