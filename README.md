# RKolors

RKolors is an example, and excercise program, it doesn't do anything particularly useful, it's a very
naive, and simplified program that utilises the [K-means clustering](https://en.wikipedia.org/wiki/K-means_clustering) algorithm to capture a subset of colours that are represented within a given image.

This implementation is extremely naive in the sense that:
1. It utilises [Lloyd's Algorithm](https://en.wikipedia.org/wiki/Lloyd%27s_algorithm)
2. It computes everything in serial (where there is plenty of opportunity to parallelise)
3. It's one of my first rust programs
4. I just wanted to get it to work (don't touch it)

Regardless; if for whatever reason you want to give it a run; read on.

## Usage 

1. git clone the repo, and build.
2. run with `rkolors -i ${input image} -o ${output image}`

You can adjust the amount of passes, and clusters that are used using the additional CLI flags.
See `rkolors --help` for more info. 

## Example

`rkolors -i sample2.png -n 3 -o output.png`

#### input

![input_image](./sample2.png)

#### output 

![output_image](./output.png)
