use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	// Enable Editor Pls
	#[arg(short, long)]
	editor_pls: bool,
}

impl Args {
	pub fn editor_pls(&self) -> bool {
		self.editor_pls
	}
}
