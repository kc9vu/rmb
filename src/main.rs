use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

static VERSION: &str = "0.2.1";
static LONG_HELP: &str = r#"remove bytes (rmb)
version v0.2.1

Usage:
  rmb <OFFSET> <INPUT> <OUTPUT>
    Reads all but the first <OFFSET> bytes from the <INPUT> and writes them to
    the <OUTPUT>, with a buffer of 4 kB.
    <OFFSET> can be negative, then only the last <OFFSET>(absolute value) bytes.
    <INPUT> and <OUTPUT> cannot be the same (case insensitive, although sometimes
    they are different files)
  rmb h[elp]
    Show help
  rmb v[ersion]
    Show version
"#;

fn main() {
    if let Err(error) = run_command() {
        eprintln!("error: {error}");
    }
}

fn run_command() -> Result<(), &'static str> {
    let mut args = std::env::args().skip(1);

    if let Some(offset) = args.next() {
        match offset.to_lowercase().as_str() {
            "help" | "h" => print_help(),
            "version" | "v" => print_version(),
            offset => {
                let seek = make_seek(offset)?;
                if let Some(input) = args.next() {
                    if let Some(output) = args.next() {
                        if input.eq_ignore_ascii_case(&output) {
                            return Err("Output cannot be the same as input");
                        }

                        rmb(seek, &input, &output)?;
                    } else {
                        return Err("No output");
                    }
                } else {
                    return Err("No input");
                }
            }
        }
    } else {
        return Err("Need a subcommnd");
    }

    Ok(())
}

fn print_help() {
    println!("{LONG_HELP}")
}

fn print_version() {
    println!("rmb v{VERSION}");
}

fn make_seek(offset: &str) -> Result<SeekFrom, &'static str> {
    match offset.parse::<i64>() {
        Ok(value) => {
            if value >= 0 {
                Ok(SeekFrom::Start(value as u64))
            } else {
                Ok(SeekFrom::End(value))
            }
        }
        Err(_) => Err("Try a valid offset"),
    }
}

fn rmb(seek: SeekFrom, input: &str, output: &str) -> Result<(), &'static str> {
    if let Ok(mut inputs) = File::open(input) {
        if let Ok(mut outputs) = File::create(output) {
            if inputs.seek(seek).is_err() {
                return Err("Failed set offset");
            }

            let mut buffer = [0; 4096];
            'main: loop {
                if let Ok(bytes_read) = inputs.read(&mut buffer) {
                    if bytes_read == 0 {
                        break 'main;
                    }
                    if outputs.write_all(&buffer[..bytes_read]).is_err() {
                        return Err("Failed write to output");
                    }
                } else {
                    return Err("Failed read from input");
                }
            }
        } else {
            return Err("Failed open output");
        }
    } else {
        return Err("Failed open input");
    }
    Ok(())
}
