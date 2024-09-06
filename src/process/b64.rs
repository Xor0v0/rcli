use crate::{get_reader, Base64Format};
use anyhow::Result;
// use base64::{
//     engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
//     Engine as _,
// };
use base64::prelude::*;

pub fn b64_encode(input: &str, format: Base64Format) -> Result<String> {
    // 如何处理作用域中返回类型不同的情况：找共性，然后用 Box 封装成 trait object
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(&buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encoded)
}

pub fn b64_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    // 解码的必然是一个字符串
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(buf),
        Base64Format::UrlSafe => BASE64_URL_SAFE_NO_PAD.decode(buf),
    }?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::{b64_decode, b64_encode, Base64Format};

    #[test]
    fn test_b64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(b64_encode(input, format).is_ok());
    }

    #[test]
    fn test_b64_decode() {
        let input = "fixture/b64.txt";
        let format = Base64Format::UrlSafe;
        assert!(b64_decode(input, format).is_ok());
    }
}
