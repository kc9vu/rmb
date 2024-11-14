use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};

static VERSION: &str = "0.2.3";
static LONG_HELP: &str = r#"remove bytes (rmb)
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
    if let Err(er) = run_command() {
        eprintln!("error: {}", er);
    }
}

fn run_command() -> Result<(), String> {
    let mut args = std::env::args().skip(1);

    match args.next().ok_or("Need a subcommand")?.to_lowercase().as_str() {
        "help" | "h" => println!("{LONG_HELP}"),
        "version" | "v" => println!("rmb v{VERSION}"),
        offset => {
            let offset = offset.parse::<i64>().map_err(|_| "Try a valid offset")?;
            let seek = make_seek(offset);

            let input = args.next().ok_or("No input".to_string())?;
            let output = args.next().ok_or("No output".to_string())?;

            if input.eq_ignore_ascii_case(&output) {
                return Err("Output cannot be the same as input".to_string());
            }

            drop(args);

            rmb(seek, &input, &output)?;
        }
    }

    Ok(())
}

fn make_seek(offset: i64) -> SeekFrom {
    if offset >= 0 {
        SeekFrom::Start(offset as u64)
    } else {
        SeekFrom::End(offset)
    }
}

fn rmb(seek: SeekFrom, input: &str, output: &str) -> Result<(), String> {
    let mut inputs = fs::File::open(input).map_err(|er| format!("Failed open input: {}", er))?;
    inputs.seek(seek).map_err(|er| format!("Failed seek file: {}", er))?;

    let mut outputs = fs::File::create(output).map_err(|er| format!("Failed create output: {}", er))?;

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = inputs.read(&mut buffer).map_err(|er| format!("Failed read from input: {}", er))?;
        if bytes_read == 0 {
            break;
        }
        outputs.write_all(&buffer[..bytes_read]).map_err(|er| format!("Failed write to output: {}", er))?;
    }

    Ok(())
}
