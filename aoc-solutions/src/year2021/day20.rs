use ndarray::Array2;

struct Image {
    default_lit: bool,
    pixels: Array2<bool>,
}

impl Image {
    pub fn enhance(&self, algorithm: [bool; 512]) -> Self {
        let mut padded_pixels = Array2::from_elem(
            (self.pixels.shape()[0] + 4, self.pixels.shape()[1] + 4),
            self.default_lit,
        );
        padded_pixels
            .slice_mut(ndarray::s![
                2..padded_pixels.shape()[0] - 2,
                2..padded_pixels.shape()[1] - 2
            ])
            .assign(&self.pixels);
        let new_pixels = ndarray::Zip::from(padded_pixels.windows((3, 3))).map_collect(|window| {
            let lookup_index = window.iter().fold(0, |lookup_index, pixel_lit| {
                lookup_index << 1 | *pixel_lit as usize
            });
            algorithm[lookup_index]
        });

        Image {
            default_lit: (!self.default_lit && algorithm[0b_000_000_000])
                || (self.default_lit && algorithm[0b_111_111_111]),
            pixels: new_pixels,
        }
    }

    pub fn count_lit(&self) -> Option<usize> {
        if !self.default_lit {
            Some(self.pixels.iter().filter(|lit| **lit).count())
        } else {
            None
        }
    }
}

fn parse_input<I: Iterator<Item = String>>(mut input_lines: I) -> ([bool; 512], Image) {
    let image_enhancement_algo =
        array_init::from_iter(input_lines.next().unwrap().bytes().map(|b| b == b'#')).unwrap();

    // Skip empty line.
    input_lines.next().unwrap();

    let mut columns = None;

    let pixels: Vec<bool> = input_lines
        .flat_map(|image_row_string| {
            if let Some(width) = columns {
                if image_row_string.len() != width {
                    panic!("expected all image rows to be of the same width");
                }
            } else {
                columns = Some(image_row_string.len());
            }
            image_row_string.into_bytes().into_iter().map(|b| b == b'#')
        })
        .collect();

    let columns = columns.expect("expected non-empty input");

    (
        image_enhancement_algo,
        Image {
            default_lit: false,
            pixels: Array2::from_shape_vec((pixels.len() / columns, columns), pixels).unwrap(),
        },
    )
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (image_enhancement_algo, mut image) = parse_input(input_lines);

    for _ in 0..2 {
        image = image.enhance(image_enhancement_algo);
    }

    image.count_lit().unwrap().to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let (image_enhancement_algo, mut image) = parse_input(input_lines);

    for _ in 0..50 {
        image = image.enhance(image_enhancement_algo);
    }

    image.count_lit().unwrap().to_string()
}
