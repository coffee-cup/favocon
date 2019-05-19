use clap::{App, Arg};
use console::{style, Emoji};
use spinners::{Spinner, Spinners};
use std::io;
use std::path::Path;

mod icon;

fn main() {
    let matches = App::new("favocon")
        .version("0.1.0")
        .author("Jake Runzer <jakerunzer@gmail.com>")
        .about("Create favicons from images")
        .arg(
            Arg::with_name("ICON")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("icon to convert into favicon"),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .help("Directory to output files to")
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("ICON").unwrap();
    let outdir = Path::new(matches.value_of("OUTPUT").unwrap_or("favocon"));

    let img = image::open(filename).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    icon::validate_img(&img).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    create_outdir(outdir).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    let sp = Spinner::new(Spinners::Dots3, "Creating favicons".into());

    let html_string = icon::create_all_favicons(&img, outdir).unwrap_or_else(|_| {
        error_out("Error creating icons");
    });

    sp.stop();

    println!(
        "\n{}Saved your favicons to {}.",
        Emoji("✨ ", ""),
        outdir.to_str().unwrap()
    );

    println!(
        "\nPlace these files at the root of your site. \
         \nCopy the following to the <head> of your HTML\n\n{}.",
        html_string
    );
}

fn create_outdir(outdir: &Path) -> io::Result<()> {
    std::fs::create_dir_all(outdir)
}

fn error_out(message: &str) -> ! {
    eprintln!("{}", style(message).red());
    ::std::process::exit(1);
}
