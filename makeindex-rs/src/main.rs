mod genind;
mod mkind;
mod qsort;
mod scanid;
mod scanst;
mod sortid;

use clap::Parser;

use mkind::CliArguments;

fn main() {
    let arguments = CliArguments::parse();

    // Call the C main function from mkind module
    let exit_code = mkind::makeindex_main(arguments);
    std::process::exit(exit_code);
}
