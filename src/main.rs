use std::io::Read as _;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "md-numbered-headers", about)]
struct Opt {
    #[structopt(short, long)]
    cleanup_only: bool,
}

fn main() {
    let opt = Opt::from_args();

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let output = if opt.cleanup_only {
        md_numbered_headers::process_cleanup(&buffer)
    } else {
        md_numbered_headers::process(&buffer)
    };
    print!("{}", output);
}
