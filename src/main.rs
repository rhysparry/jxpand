use clap::Parser;
use jxpand::cfg::{AnnotationMode, Annotations, Config};
use jxpand::Expander;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Pretty print the JSON output
    #[arg(long, short)]
    pretty: bool,
    /// Disable the count annotation
    #[arg(long)]
    no_count: bool,
    /// Disable the first annotation
    #[arg(long)]
    no_first: bool,
    /// Disable the last annotation
    #[arg(long)]
    no_last: bool,
    /// Disable the index annotation
    #[arg(long)]
    no_index: bool,
    /// The annotation mode to use
    #[arg(long, value_enum, default_value_t = AnnotationMode::Wrap)]
    mode: AnnotationMode,
    /// The prefix to use for annotations
    #[arg(long, default_value = "_")]
    prefix: String,
    /// The input file to use
    #[arg(long, short, value_parser = input_path, default_value = "-")]
    input: sio::Source,
    /// The output file to use
    #[arg(long, short, value_parser = output_path, default_value = "-")]
    output: sio::Destination,
}

impl From<&Cli> for Annotations {
    fn from(cli: &Cli) -> Self {
        let mut annotations = Annotations::default();
        if cli.no_count {
            annotations.disable_count();
        }
        if cli.no_first {
            annotations.disable_first();
        }
        if cli.no_last {
            annotations.disable_last();
        }
        if cli.no_index {
            annotations.disable_index();
        }
        annotations
    }
}

impl From<&Cli> for Config {
    fn from(cli: &Cli) -> Self {
        Config::new(Annotations::from(cli), cli.prefix.clone(), cli.mode.clone())
    }
}

mod sio {
    use std::io;
    use std::path::PathBuf;

    #[derive(Clone, Debug)]
    pub enum Source {
        Stdin,
        File(PathBuf),
    }

    #[derive(Clone, Debug)]
    pub enum Destination {
        Stdout,
        File(PathBuf),
    }

    impl TryFrom<&str> for Source {
        type Error = io::Error;

        fn try_from(s: &str) -> Result<Self, io::Error> {
            if s == "-" {
                Ok(Source::Stdin)
            } else {
                Ok(Source::File(PathBuf::from(s).canonicalize()?))
            }
        }
    }

    impl TryFrom<&str> for Destination {
        type Error = io::Error;

        fn try_from(s: &str) -> Result<Self, io::Error> {
            if s == "-" {
                Ok(Destination::Stdout)
            } else {
                Ok(Destination::File(PathBuf::from(s)))
            }
        }
    }

    impl Source {
        pub fn open(&self) -> io::Result<Box<dyn io::BufRead>> {
            match self {
                Source::Stdin => Ok(Box::new(io::stdin().lock())),
                Source::File(path) => Ok(Box::new(io::BufReader::new(std::fs::File::open(path)?))),
            }
        }
    }

    impl Destination {
        pub fn open(&self) -> io::Result<Box<dyn io::Write>> {
            match self {
                Destination::Stdout => Ok(Box::new(io::stdout().lock())),
                Destination::File(path) => {
                    Ok(Box::new(io::BufWriter::new(std::fs::File::create(path)?)))
                }
            }
        }
    }
}

fn input_path(s: &str) -> Result<sio::Source, String> {
    match sio::Source::try_from(s) {
        Ok(source) => Ok(source),
        Err(e) => Err(e.to_string()),
    }
}

fn output_path(s: &str) -> Result<sio::Destination, String> {
    match sio::Destination::try_from(s) {
        Ok(source) => Ok(source),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.run() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

impl Cli {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let config = Config::from(self);

        let mut input = self.input.open()?;

        let input = serde_json::from_reader(&mut input)?;

        let expander = jxpand::JsonExpander::new(config);
        let expanded = expander.expand(input);
        self.write_json(&expanded)?;
        Ok(())
    }

    fn write_json(&self, value: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let output = self.output.open()?;
        if self.pretty {
            serde_json::to_writer_pretty(output, value)?;
        } else {
            serde_json::to_writer(output, value)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
