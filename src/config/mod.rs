extern crate docopt;

mod version;

use self::docopt::Docopt;


pub const USAGE: &'static str = "
Nagios checker for graphite api.

Usage:
  check_graphite [options] <metric>
  check_graphite (-h | --help)
  check_graphite --version

Arguments:
  METRIC  the graphite metric expression

Options:
  -h, --help               show this message
  -v, --version            show version
  --debug                  enable debug
  -p, --url URL            the graphite-api HTTP endpoint
  -u, --user USER          username used for basic authentication
  -p, --password PASSWORD  password used for basic authentication
  -n, --ignore-none        ignore none value
  -l, --ignore-last-none IGNORELASTNONE
                           ignore the last N values of None
                           [default: 0]
  -f, --from FROM          time frame for which to query metrics
                           [default: -5min]
  -U, --until UNTIL        end time for data [default: now]
  -t, --timeout TIMEOUT    timeout after which the metric should be
                           considered unknown [default: 5]
  -w, --warning WARNING    warning threshold for the metric
  -c, --critical CRITICAL  threshold for the metric

site: <http://github.com/jaypei/check_graphite>
";

#[derive(Debug, RustcDecodable)]
pub struct Options {
    pub arg_metric: String,
    pub flag_version: bool,
    pub flag_debug: bool,
    pub flag_url: String,
    pub flag_user: String,
    pub flag_password: String,
    pub flag_ignore_none: bool,
    pub flag_from: String,
    pub flag_until: String,
    pub flag_timeout: u64,
    pub flag_warning: f64,
    pub flag_critical: f64,
    pub flag_ignore_last_none: u64,
}

lazy_static! {
    pub static ref ARGS: Options = {
        let args: Options = Docopt::new(USAGE)
            .and_then(|d| d.decode())
            .unwrap_or_else(|e| e.exit());
        if args.flag_version {
            println!("v{}", version::VERSION);
            ::std::process::exit(0);
        }
        args
    };
}
