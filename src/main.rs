#![feature(io_error_more)]
use std::{io::ErrorKind, process::ExitCode};

macro_rules! fail {
    () => {
        fail!("<infile>")
    };
    ($usage:literal) => {{
        eprintln!(concat!("usage: erad ", comat::comat!($usage)));
        return ExitCode::FAILURE;
    }};
}

fn main() -> ExitCode {
    let Some(out) = std::env::args().nth(1) else {
        fail!();
    };
    println!(
        "{:?}",
        match std::fs::read(out).map_err(|e| e.kind()) {
            Err(ErrorKind::IsADirectory) => fail!("<valid {red}{italic}file{reset}>"),
            Err(ErrorKind::PermissionDenied) => fail!("<{red}readable filename{reset}>"),
            Err(_) => fail!("<{red}infile{reset}>"),
            Ok(x) => x,
        }
    );
    ExitCode::SUCCESS
}
