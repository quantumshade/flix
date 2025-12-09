use clap::Subcommand;

#[derive(Subcommand)]
pub enum AddCommand {
	/// Process a flix collection
	Collection {
		#[arg(value_name = "TITLE")]
		title: String,
		#[arg(value_name = "OVERVIEW")]
		overview: String,
	},
}
