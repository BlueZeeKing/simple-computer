use std::io::stdout;

use simple_computer::{ SimpleComputer, commands::Commands };

fn main() {
    SimpleComputer::new(
        Commands::parse_commands("plus\nplus\nplus\nminus\nminus\nminus\nback 6\noutput\nback 1"),
        &mut stdout().lock()
    ).for_each(drop);
}