use std::io::{stdout, Write};
use std::path::PathBuf;

use anyhow::bail;
use clap::Parser;
use colored::*;
use image::{GenericImageView, ImageReader};

#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
struct Cli {
    /// The name of the Pokemon to show.
    poke_name: String,

    #[arg(long)]
    shiny: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let kind = if cli.shiny {
        "shiny"
    } else {
        "regular"
    };

    let poke_path = PathBuf::from(format!("assets/pokemon-gen8/{}/{}.png", kind, cli.poke_name));
    if !poke_path.is_file() {
        // TODO: Print unknown pokemon picture if not found!
        bail!("Pokemon {} not found!", cli.poke_name);
    }

    println!("Image found at:\n{}", poke_path.display());

    let image = ImageReader::open(poke_path)?
        .decode()?;

    // Determine sprite's bounding box.
    let mut start = (image.width(), image.height());
    let mut end = (0, 0);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            if pixel[3] == 255 {
                if x < start.0 {
                    start.0 = x;
                }
                if x > end.0 {
                    end.0 = x;
                }
                if y < start.1 {
                    start.1 = y;
                }
                if y > end.1 {
                    end.1 = y;
                }
            }
        }
    }

    // Print pixels in bounding box.
    {
        // Grab stdout to avoid grabbing the global lock for each write.
        let mut stdout = stdout();
        for y in start.1..=end.1 {
            for x in start.0..=end.0 {
                let pixel = image.get_pixel(x, y);
                if pixel[3] == 255 {
                    write!(stdout, "{}", "  ".on_truecolor(pixel[0], pixel[1], pixel[2]))?;
                } else {
                    write!(stdout, "  ")?;
                }
            }
            writeln!(stdout)?;
        }
    }

    Ok(())
}
