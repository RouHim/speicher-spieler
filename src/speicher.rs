use std::process::Command;

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

use crate::kv_store;
use std::thread;
use core::time;
use std::io::Read;

pub fn begin_caching(con: &PooledConnection<SqliteConnectionManager>) {
    let url_to_cache: String = kv_store::get(&con, "caching_url");

    let youtubedl_cmd = Command::new("/home/rouven/projects/speicher-spieler/youtube-dl")
        .arg(url_to_cache)
        .arg("-f")
        .arg("mp4")
        .arg("-o")
        .arg("/home/rouven/Downloads/cache");

    let mut spawned_process = youtubedl_cmd.spawn().expect("failed to spawn process");
    let stdout = spawned_process.stdout;
    let mut process_status = spawned_process.try_wait().unwrap();

    // TODO separaetes projekt aufsetzten asynch prozess output zyklisch geparst wird
    while Ok(process_status).is_ok() {
        let take = stdout.take().unwrap().

        println!("{}", take);
        thread::sleep(time::Duration::from_millis(500));
        process_status = spawned_process.try_wait().unwrap();
    }

    let output = youtubedl_cmd
        .output()
        .expect("failed to execute process");

    let percentage_extractor_regex = "\\d{0,3}\\.\\d+%";
}