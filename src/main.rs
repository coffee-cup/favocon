use clap::{App, Arg};
use console::style;
use image::GenericImageView;

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
        .get_matches();

    let filename = matches.value_of("ICON").unwrap();

    let img = image::open(filename).unwrap_or_else(|err| {
        error_out(&*format!("{:?}", err));
    });

    validate_img(&img).unwrap_or_else(|err| {
        error_out(&*format!("{}", err));
    });

    let ico = create_favicon(&img);

    let file = std::fs::File::create("test.ico").unwrap();
    ico.write(file).unwrap();
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

fn error_out(message: &str) -> ! {
    eprintln!("{}", style(message).red());
    ::std::process::exit(1);
}
