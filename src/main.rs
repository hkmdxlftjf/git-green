use std::env;
use std::time::Duration;
use chrono::Timelike;

mod pow;
mod vcs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let once = args.iter().any(|a| a == "--once");
    let push = env::var("GIT_GREEN_PUSH").ok().map(|v| v == "1").unwrap_or(false);
    
    // Support for specifying git repository directory
    if let Ok(work_dir) = env::var("GIT_WORK_DIR") {
        if let Err(e) = env::set_current_dir(&work_dir) {
            eprintln!("Failed to change to directory {}: {}", work_dir, e);
            std::process::exit(1);
        }
        println!("Working directory set to: {}", work_dir);
    }

    if once {
        run_once(push);
        return;
    }

    loop {
        run_once(push);
        // sleep_until_next_hour();
    }
}

fn run_once(push: bool) {
    let difficulty = compute_difficulty_from_last_hash();
    println!("difficulty={}", difficulty);
    let solve = pow::solve_current_hour(difficulty);
    if let Some(r) = solve {
        pow::append_log(&r);
        vcs::commit_pow(&r, push);
    }
}

fn sleep_until_next_hour() {
    let now = chrono::Local::now();
    let next = (now + chrono::Duration::hours(1)).with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();
    let dur = next - now;
    let ms = dur.num_milliseconds().max(0) as u64;
    std::thread::sleep(Duration::from_millis(ms));
}

fn compute_difficulty_from_last_hash() -> usize {
    let prev = pow::read_last_hash();
    if prev.is_empty() {
        return 2;
    }
    if let Some(c) = prev.chars().last() {
        match c {
            '0'..='9' => (c as u8 - b'0') as usize,
            'a'..='f' => 10 + (c as u8 - b'a') as usize,
            'A'..='F' => 10 + (c as u8 - b'A') as usize,
            _ => 2,
        }
    } else {
        2
    }
}
