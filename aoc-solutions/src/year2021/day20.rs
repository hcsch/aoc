use std::collections::HashSet;

use array_init;

fn window3x3_indices(index: [i32; 2]) -> impl Iterator<Item = [i32; 2]> {
    let [center_row, center_col] = index;
    [
        [center_row - 1, center_col - 1],
        [center_row - 1, center_col],
        [center_row - 1, center_col + 1],
        [center_row, center_col - 1],
        [center_row, center_col],
        [center_row, center_col + 1],
        [center_row + 1, center_col - 1],
        [center_row + 1, center_col],
        [center_row + 1, center_col + 1],
    ]
    .into_iter()
}

struct Image {
    default_lit: bool,
    inverted_pixels: HashSet<[i32; 2]>,
}

impl Image {
    fn enhanced_pixel_value(&self, algorithm: [bool; 512], index: [i32; 2]) -> bool {
        let [center_row, center_col] = index;
        let mut lookup_index = 0;
        for row in center_row - 1..=center_row + 1 {
            for col in center_col - 1..=center_col + 1 {
                lookup_index = lookup_index << 1
                    | (self.default_lit ^ self.inverted_pixels.contains(&[row, col])) as usize;
            }
        }
        algorithm[lookup_index]
    }

    pub fn enhance(&self, algorithm: [bool; 512]) -> Self {
        if self.default_lit && !algorithm[0b_111_111_111]
            || !self.default_lit && !algorithm[0b_000_000_000]
        {
            Image {
                default_lit: false,
                inverted_pixels: self
                    .inverted_pixels
                    .iter()
                    .copied()
                    .flat_map(|unlit_pixel_i| {
                        window3x3_indices(unlit_pixel_i).filter_map(|pixel_i| {
                            self.enhanced_pixel_value(algorithm, pixel_i)
                                .then_some(pixel_i)
                        })
                    })
                    .collect(),
            }
        } else {
            Image {
                default_lit: true,
                inverted_pixels: self
                    .inverted_pixels
                    .iter()
                    .copied()
                    .flat_map(|unlit_pixel_i| {
                        window3x3_indices(unlit_pixel_i).filter_map(|pixel_i| {
                            (!self.enhanced_pixel_value(algorithm, pixel_i)).then_some(pixel_i)
                        })
                    })
                    .collect(),
            }
        }
    }

    pub fn count_lit(&self) -> Option<usize> {
        if !self.default_lit {
            Some(self.inverted_pixels.len())
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

    let lit_pixels = input_lines
        .enumerate()
        .flat_map(|(row, image_row_string)| {
            image_row_string
                .into_bytes()
                .into_iter()
                .enumerate()
                .filter_map(move |(col, b)| (b == b'#').then_some([row as i32, col as i32]))
        })
        .collect();

    (
        image_enhancement_algo,
        Image {
            default_lit: false,
            inverted_pixels: lit_pixels,
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
