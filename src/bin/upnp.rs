use human_panic::setup_panic;
use std::str::FromStr;
use structopt::StructOpt;
use tracing::info;
use upnp_rs::common::interface::IP;
use upnp_rs::discovery::search::*;
use upnp_rs::SpecVersion;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(StructOpt)]
#[structopt(name = "upnp")]
struct CommandLine {
    /// The level of logging to perform, from off to trace, default is off
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    /// The network interface to bind to, default is all
    #[structopt(long)]
    interface: Option<String>,

    /// Use IPv6 instead of the default v4
    #[structopt(long, short = "6")]
    use_ipv6: bool,

    /// The UPnP version to use, 1.0, 1.1, or 2.0, default is 1.0
    #[structopt(long, short = "V")]
    spec_version: Option<String>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Issue a multicast search to find devices
    Search {
        /// The UPnP search target (all, root, device:{id}, device-type:{id}, service-type:{id}),
        /// default is root
        #[structopt(long, short)]
        search_target: Option<CLSearchTarget>,

        /// A domain to use in constructing device and service type targets, default is the UPnP
        /// domain.
        #[structopt(long, short)]
        domain: Option<String>,

        /// The maximum wait time, in seconds, for devices to respond to multicast, default 2 .
        #[structopt(long, short = "w")]
        max_wait: Option<u8>,
    },
    /// Listen for device notifications
    Listen,
}

#[derive(Debug)]
pub enum CLSearchTarget {
    All,
    RootDevice,
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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl FromStr for CLSearchTarget {
    type Err = CommandLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(CLSearchTarget::RootDevice)
        } else if s == "all" {
            Ok(CLSearchTarget::All)
        } else if s == "root" {
            Ok(CLSearchTarget::RootDevice)
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
        } => do_search(
            parse_version(args.spec_version),
            args.interface,
            if args.use_ipv6 { IP::V6 } else { IP::V4 },
            search_target,
            domain,
            max_wait,
        ),
        Command::Listen => do_listen(),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_tracing(verbosity: i8) {
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    let log_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::ERROR,
        2 => LevelFilter::WARN,
        3 => LevelFilter::INFO,
        4 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive(
            format!("{}={}", module_path!(), log_level)
                .parse()
                .expect("Issue with command-line trace directive"),
        )
        .add_directive(
            format!("upnp={}", log_level)
                .parse()
                .expect("Issue with library trace directive"),
        );
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global default tracing subscriber");
    info!("Log level set to `LevelFilter::{:?}`", log_level);
}

fn parse_version(version: Option<String>) -> SpecVersion {
    if let Some(s) = version {
        if s == "1.0".to_string() {
            SpecVersion::V10
        } else if s == "1.1".to_string() {
            SpecVersion::V11
        } else if s == "2.0".to_string() {
            SpecVersion::V20
        } else {
            SpecVersion::default()
        }
    } else {
        SpecVersion::default()
    }
}

fn do_search(
    spec_version: SpecVersion,
    bind_to_interface: Option<String>,
    ip_version: IP,
    search_target: Option<CLSearchTarget>,
    domain: Option<String>,
    max_wait_time: Option<u8>,
) {
    let mut options = Options::default_for(spec_version);
    options.network_interface = bind_to_interface;
    options.network_version = Some(ip_version);
    if let Some(search_target) = search_target {
        options.search_target = match search_target {
            CLSearchTarget::All => SearchTarget::All,
            CLSearchTarget::RootDevice => SearchTarget::RootDevice,
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
    println!(
        r#"
# UPnP Search Response
    
Search parameters

* UPnP version: {}
* Search Target: `{}`
* Network interface: {}
* Wait time: {} seconds
    
## Results "#,
        &options.spec_version,
        &options.search_target,
        match &options.network_interface {
            None => "all".to_string(),
            Some(s) => s.to_string(),
        },
        &options.max_wait_time
    );
    match search_once(options) {
        Ok(responses) => {
            for response in responses.iter() {
                println!("\n**[{}]({})**\n", response.service_name, response.location);
                println!("* Product Version: {}", response.versions.product_version());
                println!("* UPnP Version:    {}", response.versions.upnp_version());
                println!(
                    "* O/S Version:     {}",
                    response.versions.platform_version()
                );
            }
        }
        Err(error) => {
            println!("search failed with error: {:#?}", error);
        }
    }
}

fn do_listen() {}
