use human_panic::setup_panic;
use std::str::FromStr;
use structopt::StructOpt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::FmtSubscriber;
use upnp::ssdp::*;

#[macro_use]
extern crate tracing;

#[derive(Debug, StructOpt)]
#[structopt(name = "upnp")]
struct CommandLine {
    /// The level of logging to perform, from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(long, short)]
    interface: Option<String>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Search {
        #[structopt(long, short)]
        search_target: Option<CLSearchTarget>,

        #[structopt(long, short)]
        domain: Option<String>,

        #[structopt(long, short)]
        max_wait: Option<u8>,
    },
    Listen,
}

#[derive(Debug)]
pub enum CLSearchTarget {
    All,
    RootDevices,
    Device(String),
    DeviceType(String),
    ServiceType(String),
}

#[derive(Debug)]
pub enum CommandLineError {
    MissingParameter(String),
    UnexpectedParameter(String),
    InvalidParameterValue(String, String),
}

impl FromStr for CLSearchTarget {
    type Err = CommandLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(CLSearchTarget::RootDevices)
        } else if s == "all" {
            Ok(CLSearchTarget::All)
        } else if s == "root" {
            Ok(CLSearchTarget::RootDevices)
        } else if s.starts_with("device:") {
            Ok(CLSearchTarget::Device(s[7..].to_string()))
        } else if s.starts_with("device-type:") {
            Ok(CLSearchTarget::DeviceType(s[12..].to_string()))
        } else if s.starts_with("service-type:") {
            Ok(CLSearchTarget::ServiceType(s[13..].to_string()))
        } else {
            Err(CommandLineError::InvalidParameterValue(
                "search_target".to_string(),
                s.to_string(),
            ))
        }
    }
}

impl ToString for CommandLineError {
    fn to_string(&self) -> String {
        match self {
            CommandLineError::MissingParameter(p) => format!("Parameter {} required", p),
            CommandLineError::UnexpectedParameter(p) => format!("Parameter {} unnecessary", p),
            CommandLineError::InvalidParameterValue(p, v) => {
                format!("Value '{}' invalid for parameter {}", v, p)
            }
        }
    }
}

pub fn main() {
    setup_panic!();

    let args = CommandLine::from_args();

    init_tracing(args.verbose);

    match args.cmd {
        Command::Search {
            search_target,
            domain,
            max_wait,
        } => do_search(search_target, domain, max_wait),
        Command::Listen => do_listen(),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(verbosity: i8) {
    let log_level = match verbosity {
        0 => "",
        1 => "error",
        2 => "warn",
        3 => "info",
        4 => "debug",
        _ => "trace",
    };
    let filter = EnvFilter::from_default_env();
    let filter = if verbosity > 0 {
        filter.add_directive(
            format!("{}={}", module_path!(), log_level)
                .parse()
                .expect("Error with command-line trace directive"),
        )
    } else {
        filter
    };
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global default tracing subscriber");
    info!("Log level set to {}", log_level);
}

fn do_search(
    search_target: Option<CLSearchTarget>,
    domain: Option<String>,
    max_wait_time: Option<u8>,
) {
    let mut options = SearchOptions::default();
    options.network_interface = Some("en5".to_string());
    if let Some(search_target) = search_target {
        options.search_target = match search_target {
            CLSearchTarget::All => SearchTarget::All,
            CLSearchTarget::RootDevices => SearchTarget::RootDevices,
            CLSearchTarget::Device(d) => SearchTarget::Device(d),
            CLSearchTarget::DeviceType(dt) => {
                if let Some(domain) = domain {
                    SearchTarget::DomainDeviceType(domain, dt)
                } else {
                    SearchTarget::DeviceType(dt)
                }
            }
            CLSearchTarget::ServiceType(st) => {
                if let Some(domain) = domain {
                    SearchTarget::DomainServiceType(domain, st)
                } else {
                    SearchTarget::ServiceType(st)
                }
            }
        }
    }
    if let Some(max_wait_time) = max_wait_time {
        options.max_wait_time = max_wait_time;
    }
    match search(options) {
        Ok(responses) => {
            println!("search returned {} results.", responses.len());
            for (index, response) in responses.iter().enumerate() {
                println!("{}: {:#?}", index, response);
            }
        }
        Err(error) => {
            println!("search failed with error: {:#?}", error);
        }
    }
}

fn do_listen() {}
