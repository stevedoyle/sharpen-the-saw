// This problem was asked by Apple.
//
// You are given a hexadecimal-encoded string that has been XOR'd against a single char.
//
// Decrypt the message. For example, given the string:
//
// 7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f
// You should be able to decrypt it and get:
//
// Hello world from Daily Coding Problem

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Unable to decrypt")]
pub struct DecryptError;

pub fn decode(cipher: &str) -> Result<String> {
    let candidates = candidates(cipher);
    let printable_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";
    let printable_candidates = candidates
        .iter()
        .filter(|&candidate| candidate.chars().all(|c| printable_chars.contains(c)))
        .cloned()
        .collect::<Vec<_>>();
    let first_candidate = printable_candidates.first();
    match first_candidate {
        Some(candidate) => Ok(candidate.clone()),
        None => Err(DecryptError.into()),
    }
}

fn candidates(cipher: &str) -> Vec<String> {
    let cipher = hex::decode(cipher).unwrap();
    let mut result = Vec::new();
    for key in 0..=255 {
        let mut candidate = String::new();
        for c in cipher.iter() {
            candidate.push((c ^ key).into());
        }
        result.push(candidate);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            decode("7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello world from Daily Coding Problem");
    }

    #[test]
    fn empty_ciphertext() {
        let result = decode("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
}
