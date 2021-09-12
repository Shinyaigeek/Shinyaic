struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct Canvas {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    // Create a blank canvas
    // fn new(width: usize, height: usize) -> Canvas {
    //     let white = Color { r: 255, g: 255, b: 255, a: 255 };
    //     return Canvas {
    //         pixels: repeat(white).take(width * height).collect(),
    //         width: width,
    //         height: height,
    //     }
    // }
}
