use clap::{App, Arg};
use console::{style, Emoji};
use image::GenericImageView;
use std::path::Path;

// #[derive(Debug)]
// struct Result {
//     ico: &ico::IconDir;
// }

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
    let outdir = matches.value_of("OUTPUT").unwrap_or("favocon");

    let img = image::open(filename).unwrap_or_else(|err| {
        error_out(&*format!("{:?}", err));
    });

    validate_img(&img).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    let ico = create_favicon(&img);

    create_outdir(outdir).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    let outfile = Path::new(outdir);
    let outfile = outfile.join("favicon.ico");
    let file = std::fs::File::create(outfile).unwrap();
    ico.write(file).unwrap();

    println!("{}Saved your favicons to {}", Emoji("✨ ", ""), outdir)
}

fn create_favicon(img: &image::DynamicImage) -> ico::IconDir {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    let sizes = vec![48, 32, 16];

    for size in sizes {
        let new_img = img.resize_exact(size, size, image::FilterType::Nearest);
        let ico_img = ico::IconImage::from_rgba_data(size, size, new_img.raw_pixels());

        icon_dir.add_entry(ico::IconDirEntry::encode(&ico_img).unwrap());
    }

    icon_dir
}

fn validate_img(img: &image::DynamicImage) -> Result<(), &'static str> {
    let (width, height) = img.dimensions();
    if width == height {
        Ok(())
    } else {
        Err("Image must be square")
    }
}

fn create_outdir(outdir: &str) -> Result<(), String> {
    match std::fs::create_dir_all(outdir) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Error creating directory {}", outdir)),
    }
}

fn error_out(message: &str) -> ! {
    eprintln!("{}", style(message).red());
    ::std::process::exit(1);
}
