use std::io::{BufRead, Write};
use std::{fs, io};

use std::env::temp_dir;
use std::path::{Path, PathBuf};
use crate::gpt::Message;

const TMP_FILE: &str = "gptask.json";
const TTL_ENV_VAR: &str = "GPTASK_TTL_SECONDS";
const DEFAULT_TTL_SECONDS: u64 = 15 * 60;

pub fn get_tmp_file() -> PathBuf {
    let mut path = temp_dir();
    path.push(TMP_FILE);
    match Path::new(&path).exists() {
        true => path,
        false => {
            std::fs::File::create(&path).expect("Could not create tmp file");
            path
        }
    }
}

pub struct ContextManager {
    path: PathBuf,

}

impl ContextManager {
    pub fn new() -> ContextManager {
        ContextManager {
            path: get_tmp_file(),
        }
    }

    fn should_load_context(&self) -> bool {
        let ttl = self.ttl();
        if ttl == 0 {
            return false;
        }

        let metadata = match fs::metadata(&self.path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let modified = match metadata.modified() {
            Ok(m) => m,
            Err(_) => return false,
        };

        match modified.elapsed() {
            Ok(e) => {
                return e.as_secs() < ttl;
            },
            Err(_) => false,
        }
    }

    fn read_tmp_file(&self) -> Result<Vec<Message>, std::io::Error> {
        let file = fs::File::open(&self.path).unwrap(); 
        let mut messages = Vec::new();
        for line in io::BufReader::new(file).lines() {
            let msg = match serde_json::from_str(&line.unwrap()) {
                Ok(m) => m,
                Err(_) => continue,
            };
            messages.push(msg);
        }
        Ok(messages)    
    }

    #[allow(unused_must_use)]
    pub fn read_context(&self) -> Vec<Message> {
        match self.should_load_context() {
            true => match self.read_tmp_file() {
                Ok(m) => m,
                Err(_) => Vec::new(),
            },
            false => {
                fs::File::create(&self.path); // truncate file
                Vec::new() 
            },
        }
    }
    #[allow(unused_must_use)]
    pub fn write_context(&self, messages: &[Message]) {

        let mut file = fs::OpenOptions::new().append(true).open(&self.path).unwrap();
        for msg in messages {
            let json = match serde_json::to_string(msg) {
                Ok(j) => j,
                Err(_) => return,
            };
            writeln!(file, "{}", json);
        }
    }

    fn ttl(&self) -> u64 {
        let seconds = match std::env::var(TTL_ENV_VAR) {
            Ok(s) => match s.parse::<u64>() {
                Ok(s) => s,
                Err(_) => DEFAULT_TTL_SECONDS,
            },
            Err(_) => DEFAULT_TTL_SECONDS,
        };
        seconds
    }
}
