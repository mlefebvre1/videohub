use clap::Parser;
use videohub_proto::protocol::{Label, Route};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short = 'i',
        long = "ip-address",
        help = "IPv4 Address of the videohub device. Ex: -i 10.0.0.1"
    )]
    pub ip_address: String,
    #[clap(
        long,
        help = "Change an output label. For example, to change the label of the port 1 to 'my new label' use: 1=\"my new label\" "
    )]
    pub output_label: Option<Label>,
    #[clap(
        long,
        help = "Change an input label. For example, to change the label of the port 2 to 'foo' use: 2=foo "
    )]
    pub input_label: Option<Label>,
    #[clap(
        short = 'o',
        long = "output-route",
        help = "Route an Input to an Output: To route the input 15 to the output 40 write: 15=40 "
    )]
    pub output_route: Option<Route>,
    #[clap(
        short = 'd',
        long = "display",
        help = "Display Videohub info and current state",
        action
    )]
    pub display: bool,
    #[clap(short = 'u', long = "unlock", help = "Unlock a given port")]
    pub unlock: Option<usize>,
    #[clap(short = 'l', long = "lock", help = "Lock a given port")]
    pub lock: Option<usize>,
}

impl Cli {
    pub fn get() -> Self {
        Self::parse()
    }
}
