extern crate regex;
extern crate term_painter;

mod parse;
mod print;
mod wrap;

use term_painter::{Color::*, ToStyle};

fn main() {
	let mut op = parse::MessageIter::new();

	for i in &mut op {
		for l in i {
			l.print();
		}
	}

	println!(
		"\n~~~ {} : {} ~~~",
		Red.bold().paint(op.errors.to_string()),
		Yellow.bold().paint(op.warnings.to_string())
	);
}
