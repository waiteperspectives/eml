mod eventmodel;
mod ingest;
mod parse;
mod svg;
mod utils;

use clap::Parser;
use parse::parse;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use svg::*;
use utils::newid;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// eml input: either stdin or filepath
    #[clap(value_parser, default_value = "-")]
    input: String,

    /// eml output: either stdout or filepath
    #[clap(value_parser, default_value = "-")]
    output: String,
}

fn read_input(input: &str) -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    let mut rdr: Box<dyn io::Read> = match input {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(input)?),
    };
    rdr.read_to_string(&mut buf)?;
    Ok(buf)
}

fn write_output(output: &str, target: &str) -> Result<(), Box<dyn Error>> {
    let mut writer: Box<dyn io::Write> = match target {
        "-" => Box::new(io::stdout()),
        _ => Box::new(File::create(&Path::new(target))?),
    };
    writer.write(output.as_bytes())?;
    Ok(())
}

fn process(input: &str) -> Result<String, Box<dyn Error>> {
    let mut doc = SvgDocument {
        id: newid(),
        width: 1000f64,
        height: 1000f64,
        cards: Vec::new(),
        arrows: Vec::new(),
        swimlane: Swimlane::new(),
    };
    let model = parse(input)?;
    doc.ingest_expressions(model.expressions);
    let config = SvgConfig {
        pad: 150f64,
        card_width: 300f64,
        card_height: 150f64,
    };
    doc.set_dimensions(&config);
    let svg_string = doc.render();
    Ok(svg_string)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input = read_input(args.input.as_str())?;
    let output = process(&input)?;
    write_output(&output, args.output.as_str())?;
    Ok(())
}
