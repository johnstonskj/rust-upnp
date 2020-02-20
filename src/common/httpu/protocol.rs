/*!
Details for parsing HTTP-like messages.
*/

pub const NAME: &str = "HTTP";

pub const VERSION: &str = "1.1";

pub const ANY_RESOURCE: &str = "*";

pub const HEADER_SEP: &str = ":";

pub const LINE_SEP: &str = "\r\n";

pub const DEFAULT_TIMEOUT: u64 = 2;

pub const BUFFER_SIZE: usize = 1500;
