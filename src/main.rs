use clap::{App, Arg};
use console::{style, Emoji};
use image::GenericImageView;
use std::path::Path;

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

    validate_img(&img).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    create_outdir(outdir).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    let ico_sizes = vec![16, 32, 48];
    let png_sizes = vec![16, 32];

    let ico_dir = create_favicon(ico_sizes, &img);

    let icofile_path = outdir.join("favicon.ico");
    let icofile = std::fs::File::create(&icofile_path).unwrap();
    ico_dir.write(icofile).unwrap();

    for size in png_sizes {
        let path = outdir.join(&*format!("favicon-{}x{}.png", size, size));
        let new_img = img.resize_exact(size, size, image::FilterType::Nearest);
        new_img.save(&path).unwrap();
    }

    println!(
        "{}Saved your favicons to {}",
        Emoji("✨ ", ""),
        outdir.to_str().unwrap()
    )
}

fn create_favicon(sizes: Vec<u32>, img: &image::DynamicImage) -> ico::IconDir {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

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

fn create_outdir(outdir: &Path) -> Result<(), String> {
    match std::fs::create_dir_all(outdir) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "Error creating directory {}",
            outdir.to_str().unwrap()
        )),
    }
}

fn error_out(message: &str) -> ! {
    eprintln!("{}", style(message).red());
    ::std::process::exit(1);
}
