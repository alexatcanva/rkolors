use image::Rgb;

use super::Centroid;

/// calculate_3d_distance performs a euclidean distance calcuclation
/// over the colors.
#[inline(always)]
pub(crate) fn calculate_3d_distance(a: &Rgb<u8>, b: &Centroid) -> u8 {
    let mut res: Vec<u32> = vec![0; 3];
    for i in 0..3 {
        res[i] = (a.0[i] as i32 - b.mean.0[i] as i32).pow(2) as u32
    }

    // A fair bit of type casting here, it's less than ideal and we'll probably
    // encounter rouding issues, but it'll give us a nice-ish result

    return f32::sqrt(res.iter().sum::<u32>() as f32).floor() as u8;
}
