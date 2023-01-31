pub fn hsl_to_tuple(color_hsl: &colorsys::Hsl) -> (u16, u16, u16) {
    let color_rgb = colorsys::Rgb::from(color_hsl);
    rgb_to_tuple(&color_rgb)
}

pub fn rgb_to_tuple(color_rgb: &colorsys::Rgb) -> (u16, u16, u16) {
    let tuple: (u16, u16, u16) = color_rgb.into();
    let result = multiply_tuple(tuple, 257);
    println!("{:?}", result);
    result
}

fn multiply_tuple(tuple: (u16, u16, u16), x: u16) -> (u16, u16, u16) {
    let mut result = (0, 0, 0);
    result.0 += tuple.0 * x;
    result.1 += tuple.1 * x;
    result.2 += tuple.2 * x;
    result
}
