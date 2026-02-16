#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


mod simple {
    use clap::{Parser, Subcommand};
    use std::path::PathBuf;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        /// Optional name to operate on
        name: Option<String>,

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]    
        config: Option<PathBuf>,

        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        debug: u8,

        #[command(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Does testing shit
        Test {
            /// lists test values
            #[arg(short, long)]
            list: bool,
        }
    }

    pub fn run() {
        let cli = Cli::parse();

        if let Some(name) = cli.name.as_deref() {
            println!("Value for name: {name}");
        }

        if let Some(config_path) = cli.config.as_deref() {
            println!("Value for config: {}", config_path.display());
        }

        match cli.debug {
            0 => println!("Debug mode is off"),
            1 => println!("Debug mode is kind of on"),
            2 => println!("Debug mode is on"),
            _ => println!("Don't be crazy!"),
        }

        match &cli.command {
            Some(Commands::Test { list }) => {
                if *list {
                    println!("Printing testing lists ...");
                } else {
                    println!("Not printing testing lists ...");
                }
            },
            _ => {}
        }
    }
}

mod config_parser {
    use clap::Parser;

    #[derive(Parser)]
    // #[command(name = "MyApp")]
    // #[command(version = "1.0")]
    // #[command(about = "Does awesome things", long_about = None)]
    #[command(version, about, long_about = None)]  // Read from `Cargo.toml`
    #[command(next_line_help = true)]
    struct Cli {
        #[arg(long)]
        two: String,
        #[arg(long)]
        one: String,
    }

    pub fn run() {
        let cli = Cli::parse();

        println!("two: {:?}", cli.two);
        println!("one: {:?}", cli.one);
    }
}

mod add_args_pos {
    use clap::Parser;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        name: Vec<String>,
    }

    pub fn run() {
        let cli = Cli::parse();

        println!("name: {:?}", cli.name);
    }
}

mod add_args_opt {
    use clap::Parser;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        // #[arg(short = 'n', long = "name")]
        #[arg(short, long)]
        name: String,  // or use Vec<String> to accept multiple occurrences
    }

    pub fn run() {
        let cli = Cli::parse();

        println!("name: {:?}", cli.name);
    }
}

mod add_args_flag {
    use clap::Parser;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        // #[arg(short = 'n', long = "name")]
        #[arg(short, long)]
        verbose: bool,
    }

    pub fn run() {
        let cli = Cli::parse();

        println!("verbose: {:?}", cli.verbose);
    }
}

mod add_args_flag_multi {
    use clap::Parser;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    }

    pub fn run() {
        let cli = Cli::parse();

        println!("verbose: {:?}", cli.verbose);
    }
}

mod add_args_optional {
    use clap::Parser;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        name: Option<String>,
    }

    pub fn run() {
        let cli = Cli::parse();

        println!("name: {:?}", cli.name);
    }
}

mod add_args_default {
    use clap::Parser;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        #[arg(default_value_t = 2020)]
        port: u16,
    }

    pub fn run() {
        let cli = Cli::parse();

        println!("port: {:?}", cli.port);
    }
}

mod add_sub_variant {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    #[command(propagate_version = true)]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Adds files to app
        Add { name: Option<String> },
    }

    pub fn run() {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Add { name } => {
                println!("'add' subcommand was used, name is: {name:?}");
            }
        }
    }
}

mod add_sub_struct {
    use clap::{Parser, Subcommand, Args};

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    #[command(propagate_version = true)]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Adds files to app
        Add(AddArgs),
    }

    #[derive(Args)]
    struct AddArgs {
        name: Option<String>,
    }

    pub fn run() {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Add(name) => {
                println!("'add' subcommand was used, name is: {:?}", name.name);
            }
        }
    }
}


fn main() {
    // simple::run();
    // config_parser::run();
    // add_args_pos::run();
    // add_args_opt::run();
    // add_args_flag::run();
    // add_args_flag_multi::run();
    // add_args_optional::run();
    // add_args_default::run();
    // add_sub_variant::run();
    add_sub_struct::run();
}

