// This problem was asked by Paypal.
//
// Read this Wikipedia article on Base64 encoding.
//
// Implement a function that converts a hex string to base64.
//
// For example, the string:
//
// deadbeef
// should produce:
//
// 3q2+7w==

use anyhow::Result;
use base64::prelude::*;

pub fn base64(input: &str) -> Result<String> {
    let bytes = hex::decode(input)?;
    Ok(BASE64_STANDARD.encode(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = base64("deadbeef").unwrap();
        assert_eq!(result, "3q2+7w==");
    }

    #[test]
    fn not_hex() {
        let result = base64("deadbeeg");
        assert!(result.is_err());
    }
}
