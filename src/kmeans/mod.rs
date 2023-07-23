mod math;
pub mod options;

use std::vec;

use image::{DynamicImage, Rgb};
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

use self::options::InitialisationOption;

pub struct Centroid {
    mean: Rgb<u8>,
    vectors: Vec<Rgb<u8>>,
}

impl Centroid {
    fn from_random(rng: &mut ThreadRng) -> Self {
        let mut init: [u8; 3] = [0, 0, 0];
        for i in 0..3 {
            init[i] = rng.gen::<u8>();
        }

        return Centroid {
            mean: Rgb(init),
            vectors: vec![],
        };
    }

    fn from_image(img: DynamicImage, rng: &mut ThreadRng) -> Self {
        let src_image = img.to_rgb8();
        let src = src_image.pixels();
        let init = Some(src.choose(rng)).unwrap().expect("ahhhh");
        return Centroid {
            mean: init.clone(),
            vectors: vec![],
        };
    }

    fn calculate_centroid(&mut self) {
        // If we have no vectors, we can't calculate a denominator
        // and as such, we need to return the mean, as you can't
        // divide by 0.
        if self.vectors.len() == 0 {
            self.mean = self.mean;
            return;
        }
        let denominator = self.vectors.len() as u64;

        let mut pre_resultant_vector: Vec<u64> = vec![0; 3];
        self.vectors.iter().for_each(|color| {
            for i in 0..3 {
                pre_resultant_vector[i] += color.0[i] as u64
            }
        });

        let resultant_vector: Vec<u8> = pre_resultant_vector
            .iter()
            .map(|u| (u / denominator) as u8)
            .collect();

        self.mean = Rgb([
            resultant_vector[0],
            resultant_vector[1],
            resultant_vector[2],
        ]);
    }
}

/// calculate_kmeans_grouping will calculate the collection of average colours in an image
/// using the naive kmeans grouping algorithm
pub fn calculate_kmeans_grouping(
    input_image: DynamicImage,
    passes: usize,
    groups: usize,
    init_option: InitialisationOption,
) -> Vec<Rgb<u8>> {
    // we need a random generator to create our initial cluster centroids
    let mut rng = rand::thread_rng();

    // naive initialisation, means that we are just going to select n * groups set of random
    // points in 3d space between u8:0, and u8:1
    let mut all_centroids: Vec<Centroid> = vec![];
    for _ in 0..groups {
        all_centroids.push(match init_option {
            InitialisationOption::Random => Centroid::from_random(&mut rng),
            InitialisationOption::FromImage => Centroid::from_image(input_image.clone(), &mut rng),
        });
    }

    let rgb8p = input_image.to_rgb8();

    // for each pass, we need to assign all the vectors (colours) to the nearest
    // cluster.
    for _ in 0..passes {
        // for each pixel
        for p in rgb8p.pixels() {
            // find the closest centroid
            let closest = all_centroids
                .iter_mut()
                .min_by(|x, y| {
                    let a = math::calculate_3d_distance(p, x);
                    let b = math::calculate_3d_distance(p, y);
                    a.cmp(&b)
                })
                .unwrap();

            // assign the pixel/vector/colour to the centroid found.
            closest.vectors.push(p.clone());
        }

        // now that we have assigned all the pixels, let's calculate
        // the mean of each centroid, and reset the centroid's collections.
        for centroid in all_centroids.iter_mut() {
            centroid.calculate_centroid();
            centroid.vectors.clear();
        }
    }

    return all_centroids.iter().map(|c| c.mean).collect();
}
