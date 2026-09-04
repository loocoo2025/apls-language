#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let result = apls_cli::run(std::env::args_os());
    let mut stdout = std::io::stdout().lock();
    let mut stderr = std::io::stderr().lock();
    ExitCode::from(apls_cli::write_command_result(
        &result,
        &mut stdout,
        &mut stderr,
    ))
}
