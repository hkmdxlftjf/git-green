use sha2::{Digest, Sha256};
use chrono::{Local, Datelike, Timelike};
use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowRecord {
    pub timestamp: String,
    pub challenge: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash_hex: String,
    pub difficulty: usize,
}

pub fn solve_current_hour(k: usize) -> Option<PowRecord> {
    let now = Local::now();
    let challenge = format!("{}{:02}{:02}{:02}", now.year(), now.month(), now.day(), now.hour());
    let prev_hash = read_last_hash();
    let mut nonce: u64 = 0;
    loop {
        let mut hasher = Sha256::new();
        hasher.update(challenge.as_bytes());
        hasher.update(nonce.to_be_bytes());
        hasher.update(prev_hash.as_bytes());
        let digest = hasher.finalize();
        let hex = format!("{:x}", digest);
        if leading_zeros(&hex) >= k {
            return Some(PowRecord {
                timestamp: now.to_rfc3339(),
                challenge,
                prev_hash,
                nonce,
                hash_hex: hex,
                difficulty: k,
            });
        }
        nonce = nonce.wrapping_add(1);
        if nonce == 0 { return None; }
    }
}

pub fn append_log(rec: &PowRecord) {
    let path = "pow.log";
    let mut f = OpenOptions::new().create(true).append(true).open(path).unwrap();
    let line = serde_json::to_string(rec).unwrap();
    let _ = writeln!(f, "{}", line);
}

pub fn leading_zeros(hex: &str) -> usize {
    let mut count = 0;
    for c in hex.chars() {
        if c == '0' { count += 1; } else { break; }
    }
    count
}

pub fn read_last_hash() -> String {
    let path = "pow.log";
    if let Ok(f) = OpenOptions::new().read(true).open(path) {
        let reader = BufReader::new(f);
        if let Some(Ok(line)) = reader.lines().last() {
            if let Ok(rec) = serde_json::from_str::<PowRecord>(&line) {
                return rec.hash_hex;
            }
        }
    }
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zeros_count() {
        assert_eq!(leading_zeros("000abc"), 3);
        assert_eq!(leading_zeros("abc"), 0);
    }
}

