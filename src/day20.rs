use crate::utils;

pub fn day20(input_lines: &[String]) -> (u64, u64) {
    let input_sections = utils::group_lines_split_by_empty_line(input_lines);
    let algorithm: Vec<bool> = input_sections[0][0].chars().map(|c| c == '#').collect();
    let mut pixels: Vec<Vec<bool>> = Vec::new();
    for line in input_sections[1] {
        pixels.push(line.chars().map(|c| c == '#').collect());
    }
    let mut default_pixel = false;

    for _ in 0..2 {
        pixels = enhance_image(pixels, &algorithm, default_pixel);
        if algorithm[0] { default_pixel = !default_pixel; }
    }
    let part1 = count_lit_pixels(&pixels);

    for _ in 2..50 {
        pixels = enhance_image(pixels, &algorithm, default_pixel);
        if algorithm[0] { default_pixel = !default_pixel; }
    }
    let part2 = count_lit_pixels(&pixels);

    (part1, part2)
}

fn count_lit_pixels(pixels: &[Vec<bool>]) -> u64 {
    pixels.iter().map(|row| row.iter().filter(|pixel| **pixel).count() as u64).sum::<u64>()
}

fn enhance_image(pixels: Vec<Vec<bool>>, algorithm: &[bool], default_pixel: bool) -> Vec<Vec<bool>> {
    let pixels = grow_image(pixels, default_pixel);
    let mut new_pixels = pixels.clone();
    for row in 0..new_pixels.len() {
        for col in 0..new_pixels.len() {
            new_pixels[row][col] = get_enhanced_pixel(&pixels, algorithm, row, col, default_pixel);
        }
    }
    new_pixels
}

fn _print_image(pixels: &[Vec<bool>]) {
    for row in pixels {
        for pixel in row {
            if *pixel { print!("#"); } else { print!("."); }
        }
        println!();
    }
}

fn get_pixel(pixels: &[Vec<bool>], row: usize, col: usize, default_pixel: bool) -> bool {
    if (0..pixels.len()).contains(&row) && (0..pixels[0].len()).contains(&col) {
        pixels[row][col]
    } else {
        default_pixel
    }
}

fn get_enhanced_pixel(pixels: &[Vec<bool>], algorithm: &[bool], row: usize, col: usize, default_pixel: bool) -> bool {
    let mut index = 0usize;
    let irow = row as isize;
    let icol = col as isize;
    for g_row in (irow - 1)..=(irow + 1) {
        for g_col in (icol - 1)..=(icol + 1) {
            index <<= 1;
            if get_pixel(pixels, g_row as usize, g_col as usize, default_pixel) {
                index += 1;
            }
        }
    }
    algorithm[index]
}

fn grow_image(pixels: Vec<Vec<bool>>, default_pixel: bool) -> Vec<Vec<bool>> {
    let mut new_image: Vec<Vec<bool>> = Vec::with_capacity(pixels.len() + 2);
    let new_width = pixels[0].len() + 2;
    let empty_row = std::iter::repeat(default_pixel).take(new_width).collect::<Vec<_>>();

    new_image.push(empty_row.clone());

    for row in pixels {
        let mut new_row: Vec<bool> = Vec::with_capacity(new_width);
        new_row.push(default_pixel);
        new_row.extend(row.iter());
        new_row.push(default_pixel);
        new_image.push(new_row);
    }

    new_image.push(empty_row);

    new_image
}