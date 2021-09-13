use core::time;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use std::thread;

use lazy_static::lazy_static;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use regex::Regex;

use crate::kv_store;

lazy_static! {
    static ref PERCENTAGE_PATTERN: Regex = Regex::new(r"(?P<percentage>\d{0,3}\.\d+)%").unwrap();
}

pub fn begin_caching(con: &PooledConnection<SqliteConnectionManager>) {
    let url_to_cache: String = kv_store::get(&con, "caching_url");
    kv_store::set(&con, "is_caching", true);

    let mut cmd = Command::new("/home/rouven/projects/playground/yt-dlp")
        .arg(url_to_cache)
        .stdout(Stdio::piped())
        .spawn().expect("cannot spawn");

    let cache_thread = std::thread::spawn(move || {
        let mut reader = BufReader::new(cmd.stdout.unwrap());

        let mut buffer = [0; 48];
        let mut bytes_read = reader.read(&mut buffer);
        let mut percentage = extract_percentage(&buffer);

        while bytes_read.is_ok() && bytes_read.unwrap() > 0 {
            bytes_read = reader.read(&mut buffer);
            percentage = extract_percentage(&buffer);
            kv_store::set(&con, "caching_progress", percentage);
        }
    });

    // block until caching is done
    cache_thread.join();
    kv_store::set(&con, "is_caching", false);
}

fn extract_percentage(data_buffer: &[u8]) -> f32 {
    let str_data = std::str::from_utf8(data_buffer).unwrap();
    let percentage = PERCENTAGE_PATTERN.captures(str_data)
        .map(|mat| mat.name("percentage"))
        .map(|mat| mat.unwrap().as_str().to_string())
        .unwrap_or(String::from("0"));
    percentage.parse().unwrap_or(0)
}