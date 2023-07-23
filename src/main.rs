use clap::{arg, Parser};
use image::{io::Reader as ImageReader, ImageBuffer};

pub mod kmeans;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image file to use.
    #[arg(short, long)]
    input: String,
    // Output file.
    #[arg(short, long, default_value("output.png"))]
    output: String,

    /// Limit K-Means clustering passes.
    #[arg(long, default_value_t = 5)]
    passes: usize,

    /// Number of clusters
    #[arg(short('n'), default_value_t = 5)]
    clusters: usize,
}

fn main() {
    let args = Args::parse();

    let img = ImageReader::open(args.input).unwrap().decode().unwrap();
    let output_colors = kmeans::calculate_kmeans_grouping(
        img,
        args.passes,
        args.clusters,
        kmeans::options::InitialisationOption::FromImage,
    );

    println!("Generated Colours:");
    for i in output_colors.iter() {
        println!("({}, {}, {})", i.0[0], i.0[1], i.0[2]);
    }

    let width = 500;
    let height = 500;

    let mut imgbuf = ImageBuffer::new(width, height);
    let stripe_width = 500 / args.clusters;

    for (stripe_idx, color) in output_colors.iter().enumerate() {
        let x_start = stripe_idx * stripe_width;
        let x_end = (stripe_idx + 1) * stripe_width;

        for x in x_start..x_end {
            for y in 0..height {
                imgbuf.put_pixel(x as u32, y as u32, *color);
            }
        }
    }

    imgbuf.save(args.output).expect("Failed to save image.");
}
