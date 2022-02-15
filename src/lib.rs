use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    fs::OpenOptions,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniq")
        .version("0.1.0")
        .author("Alejandro Martinez <amnaredo@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("Show counts")
                .short("c")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of("in_file").map(str::to_string).unwrap(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>4} ", value)
    } else {
        "".to_string()
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut last_line = line.clone();
    let mut line_count: usize = 0;

    // create output file if needed
    let mut output: Box<dyn Write> = match config.out_file {
        // Some(filename) => Box::new(File::create(filename)?),
        Some(filename) => Box::new(File::options().append(true).create(true).open(filename)?),
        None => Box::new(io::stdout()),
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line_count == 0 {
             last_line = line.clone();
             line_count += 1;
         //} else if line.trim() != last_line.trim() {
         } else if line.trim_end() != last_line.trim_end() { // due to ends of line
            // distinct line
            // print!("{}{}", format_field(line_count, config.count), last_line);
            output.write(format!("{}{}", format_field(line_count, config.count), last_line).as_bytes());
            last_line = line.clone();
            line_count = 1;
        } else {
            // equal line
            line_count += 1;
        }
        line.clear();
    }

    // last line of the file
    if line_count > 0 {
        // print!("{}{}", format_field(line_count, config.count), last_line);
        output.write(format!("{}{}", format_field(line_count, config.count), last_line).as_bytes());
    }

    Ok(())
}
