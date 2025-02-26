use std::process::ExitCode;

use std::io;

fn sub() -> Result<(), io::Error> {
    let encode: bool = std::env::var("ENV_ENCODE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or_default();
    rs_lz4cat::stdin2stdout_b(encode)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
