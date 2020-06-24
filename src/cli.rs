use clap::{App, Arg};

pub fn create_cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new("toc_creator")
    .about(
        "Tool to enrich a .md-file with a table of contents."
    )
    .arg(
        Arg::with_name("file")
        .short("f")
        .long("file")
        .value_name("MD_FILE")
        .help("Specifies to the path to the file which should be enriched with a table of content.")
        .takes_value(true)
    )
} 