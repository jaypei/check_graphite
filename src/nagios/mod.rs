#![allow(dead_code)]

pub type NagiosExitCode = u8;

pub const EXIT_OK: NagiosExitCode = 0;
pub const EXIT_WARNING: NagiosExitCode = 1;
pub const EXIT_CRITICAL: NagiosExitCode = 2;
pub const EXIT_UNKNOWN: NagiosExitCode = 3;
pub const EXIT_DEPENDENT: NagiosExitCode = 4;


pub fn exit(code: NagiosExitCode) {
    ::std::process::exit(code as i32);
}
