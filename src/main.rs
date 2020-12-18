#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(clippy::single_match_else)]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::{
    borrow::Cow,
    env,
    fs,
    fs::File,
    io::{stdin, stdout, BufWriter, Read, Write},
    path::PathBuf,
    process,
    process::{Command, Stdio},
};
use tracing::{error, info, level_filters::LevelFilter, warn};
use tracing_subscriber::EnvFilter;

#[macro_use]
mod macros;

fn main() {
    process::exit(main2());
}

fn main2() -> i32 {
    let logfile = BufWriter::new(
        File::create(env::temp_dir().join("rustfmt-shim.log")).expect("could not create logfile"),
    );
    let (appender, _guard) = tracing_appender::non_blocking(logfile);
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(EnvFilter::default().add_directive(LevelFilter::INFO.into()))
        .with_writer(appender)
        .init();

    info!(command_line = ?env::args().collect::<Vec<_>>(), "starting");

    if env::args().nth(1).as_deref() == Some("--install-the-shim") {
        install();
        0
    } else {
        match run() {
            Ok(()) => 0,
            Err(()) => 1,
        }
    }
}

// It's ok to panic; the user will see stderr
fn install() {
    let home = dirs::home_dir().expect("could not get home dir");
    let current_exe = env::current_exe().expect("could not get current exe");

    let mut dest_path = home
        .join(".cargo")
        .join("bin")
        .join("rustfmt")
        .into_os_string();
    if let Some(ext) = current_exe.extension() {
        dest_path.push(".");
        dest_path.push(ext);
    }

    let mut backup_path = dest_path.clone();
    backup_path.push(".bk");
    let backup_path = PathBuf::from(backup_path);
    if !backup_path.exists() {
        fs::copy(&dest_path, &backup_path).expect("could not back up existing rustfmt");
    }

    fs::copy(&current_exe, &dest_path).expect("could not copy exe");
}

// It's not ok to panic; intellij will not show stderr to the user. Make sure to
// write all errors to the logfile.
fn run() -> Result<(), ()> {
    let mut source = String::with_capacity(1024);
    let len = stdin()
        .read_to_string(&mut source)
        .map_err(|err| error!(err = ?err, "could not read from stdin"))?;
    info!(len = len, "read source from stdin");

    let toolchain = match choose_toolchain() {
        Ok(toolchain) => Cow::from(toolchain),
        Err(()) => {
            warn!("error determining toolchain; falling back to stable");
            "stable".into()
        }
    };
    info!(toolchain = &*toolchain, "chose toolchain");

    let rustfmt = Command::new("rustup")
        .arg("run")
        .arg(&*toolchain)
        .arg("rustfmt")
        .args(env::args().skip(1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| error!(err = ?err, "error running `rustup run rustfmt`"))?;
    rustfmt
        .stdin
        .as_ref()
        .unwrap()
        .write_all(source.as_bytes())
        .map_err(|err| error!(err = ?err, "could not write data to rustfmt"))?;
    let output = rustfmt
        .wait_with_output()
        .map_err(|err| error!(err = ?err, "could not read output from rustfmt"))?;
    let code = output
        .status
        .code()
        .ok_or_else(|| error!("could not get rustfmt exit code"))?;
    info!(exit_code = code, "rustfmt finished");
    if code != 0 {
        warn!(stderr = &*String::from_utf8_lossy(&output.stderr));
        return Err(());
    }
    stdout()
        .write_all(&output.stdout)
        .map_err(|err| error!(err = ?err, "could not write to stdout"))?;
    Ok(())
}

fn choose_toolchain() -> Result<String, ()> {
    let data = fs::read_to_string(".pre-commit-config.yaml")
        .map_err(|err| warn!(err = ?err, "could not read pre-commit config"))?;
    info!("read pre-commit config");
    let toolchain = match regex!(r"rustup run(?: --install)? (\S+)").captures(&data) {
        Some(m) => m[1].to_string(),
        None => {
            warn!("no match; falling back to stable toolchain");
            "stable".to_string()
        }
    };
    Ok(toolchain)
}
