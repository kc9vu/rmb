use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

static VERSION: &str = "0.2.2";
static LONG_HELP: &str = r#"remove bytes (rmb)
version v0.2.2

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
        eprintln!("error: {}", error);
    }
}

fn run_command() -> Result<(), &'static str> {
    let mut args = std::env::args().skip(1);

    let offset = args.next().ok_or("Need a subcommnd")?;

    match offset.to_lowercase().as_str() {
        "help" | "h" => print_help(),
        "version" | "v" => print_version(),
        offset => {
            let seek = make_seek(offset)?;
            let input = args.next().ok_or("No input")?;
            let output = args.next().ok_or("No output")?;

            if input.eq_ignore_ascii_case(&output) {
                return Err("Output cannot be the same as input");
            }
            drop(args);

            rmb(seek, &input, &output)?;
        }
    }

    Ok(())
}

fn print_help() {
    println!("{}", LONG_HELP);
}

fn print_version() {
    println!("rmb v{}", VERSION);
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
    let mut inputs = File::open(input).or(Err("Failed open input"))?;
    if inputs.seek(seek).is_err() {
        return Err("Failed set offset");
    }

    let mut outputs = File::create(output).or(Err("Failed create output"))?;

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = inputs.read(&mut buffer).or(Err("Failed read from input"))?;

        if bytes_read == 0 {
            break;
        }
        if outputs.write_all(&buffer[..bytes_read]).is_err() {
            return Err("Failed write to output");
        }
    }

    Ok(())
}
