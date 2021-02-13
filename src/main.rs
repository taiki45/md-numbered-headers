use std::io::Read as _;
use std::process::exit;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "md-numbered-headers", about)]
struct Opt {
    /// Remove numbered headers.
    #[structopt(short, long)]
    cleanup_only: bool,

    /// Which depth md-numbered-headers starts header numbering counting and adding.
    /// If set 2, it starts counting from depth 2 headers (`##`).
    #[structopt(short, long, default_value = "2")]
    start_depth: usize,

    /// Which depth md-numbered-headers ends header numbering counting and adding.
    /// If set 5, it ends counting and adding numbers from depth 5 headers (`#####`).
    #[structopt(short, long, default_value = "5")]
    end_depth: usize,

    /// Whether reset numbered header counting or not with higher depth headers.
    /// See README for the detail behavior.
    #[structopt(long)]
    reset_with_higher_depth: bool,
}

fn main() {
    let opt = Opt::from_args();
    validate_opt(&opt);
    let opt = convert_opt(opt);

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let output = md_numbered_headers::process(&buffer, opt);
    print!("{}", output);
}

fn validate_opt(opt: &Opt) {
    if opt.start_depth >= opt.end_depth {
        eprintln!("option end_depth must be greater than start_depth.");
        exit(1);
    }
}

fn convert_opt(opt: Opt) -> md_numbered_headers::Opt {
    match opt {
        Opt {
            cleanup_only,
            start_depth,
            end_depth,
            reset_with_higher_depth,
        } => md_numbered_headers::Opt {
            cleanup_only,
            start_depth,
            end_depth,
            reset_with_higher_depth,
        },
    }
}
