use bitpet::cli;

fn main() {
    if let Err(error) = cli::run(std::env::args().skip(1)) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
