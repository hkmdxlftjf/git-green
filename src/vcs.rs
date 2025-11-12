use crate::pow::PowRecord;
use std::process::Command;

pub fn commit_pow(rec: &PowRecord, push: bool) {
    let msg = format!("pow: {} k={} hour={}", rec.hash_hex, rec.difficulty, rec.timestamp);
    let _ = Command::new("git").args(["add", "pow.log"]).status();
    let _ = Command::new("git").args(["commit", "-m", &msg]).status();
    if push { let _ = Command::new("git").args(["push"]).status(); }
}

