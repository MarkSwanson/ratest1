
#![feature(backtrace)]
use std::fs::File;
use std::{fmt, io::{self, Write}, str};
use std::backtrace::Backtrace;

use thiserror::Error;

pub mod r2;

fn testT() -> Result<(), RATestError> {
    let mut writer = io::BufWriter::new(File::create("abc")?);
    let s = String::new();
    let s2 = String::with_capacity(100);
    let s3 = String::new();
    let s4 = String::new();
    let s5 = String::with_capacity(10);
    let s6 = String::new();
    Ok(())
}

// thiserror: https://github.com/dtolnay/thiserror
#[derive(Error, Debug)]
pub enum RATestError {
    //#[error("data store disconnected")]
    //Disconnect(#[from] io::Error),

    #[error("{0}")]
    Message(String),
    
    #[error("I/O Error")]
    Io {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },

    #[error("Anyhow Generic std::error::Error trait")]
    AnyhowError {
        #[from]
        source: anyhow::Error,
        backtrace: Backtrace,
    },

}

pub enum RATest1Storage {
    MEMORY,
    Filename(String),
}

pub enum RATest1Kind {
    Naive,
    Chunked,
}

pub struct RATest1Builder{

    storage: RATest1Storage,
    kind: RATest1Kind,
    logger: slog::Logger,

}

impl RATest1Builder {
    pub fn new_memory(logger: slog::Logger, kind: RATest1Kind) -> Result<RATest1Builder, RATestError> {
        Ok(RATest1Builder {
            logger: logger.clone(),
            storage: RATest1Storage::MEMORY,
            kind
        })
    }
}

pub struct RATest1Data {
    pub logger: slog::Logger,
}

impl fmt::Debug for RATest1Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RATest1Data {{ }}" )
    }
}

impl RATest1Data {

    pub fn new(logger: slog::Logger, _filename: &str) -> Result<RATest1Data, RATestError> {
        Ok(RATest1Data {
            logger
        })
    }

    pub fn addpoint(mut self, x: u32, y: u32, z: u32) -> Result<(), RATestError> {
        let x_be = x.to_be_bytes();
        let y_be = y.to_be_bytes();
        let z_be = z.to_be_bytes();
        Ok(())
    }

}

pub fn atest42() -> i32 {
    r2::test2();
    42
}

pub fn b() -> i32 {
    let a = String::new();
    println!("abc {}", a);
    atest42()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

