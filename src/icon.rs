use image::GenericImageView;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use tempdir::TempDir;

fn create_icon_dir(sizes: Vec<u32>, img: &image::DynamicImage) -> io::Result<ico::IconDir> {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for size in sizes {
        let new_img = img.resize_exact(size, size, image::FilterType::CatmullRom);
        let ico_img = ico::IconImage::from_rgba_data(size, size, new_img.raw_pixels());

        icon_dir.add_entry(ico::IconDirEntry::encode(&ico_img)?);
    }

    Ok(icon_dir)
}

fn create_manifest(outdir: &Path, html_string: &mut String) -> io::Result<()> {
    let path = outdir.join("site.webmanifest");

    let manifest = r##"{
  "name": "",
  "short_name": "",
  "icons": [
    {
      "src": "/android-chrome-192x192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "/android-chrome-512x512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ],
  "theme_color": "#ffffff",
  "background_color": "#ffffff",
  "display": "standalone"
}
"##;

    let mut file = File::create(path)?;
    file.write_all(manifest.as_bytes())?;

    let link_rel = "<link rel=\"manifest\" href=\"/site.webmanifest\" />\n".to_string();
    html_string.push_str(link_rel.as_str());

    Ok(())
}

pub fn validate_img(img: &image::DynamicImage) -> Result<(), &'static str> {
    let (width, height) = img.dimensions();
    if width == height {
        Ok(())
    } else {
        Err("Image must be square")
    }
}

// Save image to tmp file so it is converted to png
pub fn convert_to_png(img: &image::DynamicImage) -> io::Result<image::DynamicImage> {
    let tmp_dir = TempDir::new("favocon").unwrap();
    let png_file_path = tmp_dir.path().join("img.png");

    let img = img.to_rgba();
    img.save(&png_file_path)?;

    match image::open(&png_file_path) {
        Ok(img) => Ok(img),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Error converting image to png",
        )),
    }
}

pub fn create_all_favicons(img: &image::DynamicImage, outdir: &Path) -> io::Result<String> {
    let ico_sizes = vec![16, 32, 48];
    let ico_dir = create_icon_dir(ico_sizes, &img)?;

    let mut html_string: String = "".to_owned();

    // Create favicon.ico
    let icofile_path = outdir.join("favicon.ico");
    let icofile = std::fs::File::create(&icofile_path)?;
    ico_dir.write(icofile)?;

    // Create png favicons
    let png_sizes = vec![16, 32, 96];
    for size in png_sizes {
        let path = outdir.join(&*format!("favicon-{}x{}.png", size, size));
        let new_img = img.resize_exact(size, size, image::FilterType::CatmullRom);
        new_img.save(&path)?;

        let link_rel = format!("<link rel=\"icon\" type=\"image/png\" sizes=\"{size}x{size}\" href=\"/favicon-{size}x{size}.png\" />\n", size=size);
        html_string.push_str(link_rel.as_str());
    }

    // Create apple touch icon
    let apple_size = 180;
    let path = outdir.join("apple-touch-icon.png");
    let new_img = img.resize_exact(apple_size, apple_size, image::FilterType::CatmullRom);
    new_img.save(&path)?;

    let mut html_string = format!(
        "{}<link rel=\"apple-touch-icon\" sizes=\"{}x{}\" href=\"/apple-touch-icon.png\" />\n",
        html_string, apple_size, apple_size
    );

    // Create android icons
    let android_sizes = vec![192, 512];
    for size in android_sizes {
        let path = outdir.join(&*format!("android-chrome-{}x{}.png", size, size));
        let new_img = img.resize_exact(size, size, image::FilterType::CatmullRom);
        new_img.save(&path)?;
    }

    // Save android site manifiest
    create_manifest(outdir, &mut html_string)?;

    Ok(html_string)
}
