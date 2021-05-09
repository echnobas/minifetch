#![feature(try_trait)]
use prettytable::{cell, ptable, row, table};
use std::{option::NoneError, process::Command};


#[cfg(target_os="linux")]
extern "C" {
    fn getppid() -> i32;
}

fn get_parent_name(ppid: i32) -> Option<String> {
    let f = std::fs::read_to_string(&format!("/proc/{}/status", ppid)).ok()?;
    f.lines()
        .next()
        .map(|line| line.split(':').nth(1).map(|s| s.trim().to_owned()))
        .flatten()
}


#[cfg(target_os = "linux")]
fn get_gpu() -> Option<String> {
    let stdout = Command::new("lspci").output().ok()?.stdout;
    let data = std::str::from_utf8(&stdout).ok()?;
    data.lines()
        .filter_map(|line| {
            if line.contains("VGA") {
                line.split(": ").nth(1).map(|gpu| gpu.trim().to_owned())
            } else {
                None
            }
        })
        .next()
}

#[cfg(target_os = "linux")]
fn get_cpu() -> Option<String> {
    let f = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    f.lines()
        .filter_map(|line| {
            if line.contains("model name") {
                line.split(": ").nth(1).map(|cpu| cpu.trim().to_owned())
            } else {
                None
            }
        })
        .next()
}

#[cfg(target_os = "linux")]
fn hostname() -> Option<String> {
    std::fs::read_to_string("/etc/hostname").ok().or_else(|| {
        let stdout = Command::new("sh")
            .args(&["-c", "hostname"])
            .output()
            .ok()?
            .stdout;
        std::str::from_utf8(&stdout)
            .ok()
            .map(|s| s.trim().to_owned())
    })
}

#[cfg(target_os = "linux")]
fn get_distro() -> Option<String> {
    let r = std::fs::read_to_string;
    r("/bedrock/etc/os_release")
        .or_else(|_| r("/etc/os-release"))
        .or_else(|_| r("/usr/lib/os-release"))
        .map(|s| s.lines().next().map(|s| s.split('=').nth(1).map(String::from)))
        .ok()
        .flatten()
        .flatten()
}

fn main() -> Result<(), NoneError> {
    ptable!(
        [bucFg=> "Device", "Data"],
        [iucFR => "Distribution", get_distro()?],
        [iucFR => "CPU", get_cpu()?],
        [iucFR => "GPU", get_gpu()?],
        [iucFR => "Hostname", hostname()?],
        [iucFR => "Terminal", get_parent_name(unsafe { getppid() })?]
    );
    Ok(())
}
