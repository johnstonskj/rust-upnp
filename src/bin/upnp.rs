use human_panic::setup_panic;
use log::LevelFilter;
use std::str::FromStr;
use structopt::StructOpt;
use upnp::ssdp::search::*;

#[macro_use]
extern crate env_logger;

#[macro_use]
extern crate log;

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
        } => do_search(args.interface, search_target, domain, max_wait),
        Command::Listen => do_listen(),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(verbosity: i8) {
    env_logger::Builder::from_default_env()
        .filter_module(
            module_path!(),
            match verbosity {
                0 => LevelFilter::Off,
                1 => LevelFilter::Error,
                2 => LevelFilter::Warn,
                3 => LevelFilter::Info,
                4 => LevelFilter::Debug,
                _ => LevelFilter::Trace,
            },
        )
        .init();
    info!("Max log filter level set to {:?}", log::max_level());
}

fn do_search(
    bind_to_interface: Option<String>,
    search_target: Option<CLSearchTarget>,
    domain: Option<String>,
    max_wait_time: Option<u8>,
) {
    let mut options = Options::default();
    options.network_interface = bind_to_interface;
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
    match search_once(options) {
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
