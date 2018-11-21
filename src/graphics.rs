use png;
use std::sync::Arc;

/// For now we only support RGB colors so we don't have to do alpha-blending in our software renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    a: u8,
}

impl Color {
    /// Create a color from (r, g, b) components.
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn invisible() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    pub fn is_visible(self) -> bool {
        self.a > 0
    }
    pub fn black() -> Color {
        Color::rgb(0, 0, 0)
    }
    pub fn white() -> Color {
        Color::rgb(0xff, 0xff, 0xff)
    }
    pub fn grayscale_byte(&self) -> u8 {
        (self.luminance() * 255.0) as u8
    }
    pub fn to_grayscale(&self) -> Color {
        let gray = self.grayscale_byte();
        Color::rgb(gray, gray, gray)
    }

    fn float_red(&self) -> f64 {
        self.r as f64 / 255.0
    }
    fn float_green(&self) -> f64 {
        self.g as f64 / 255.0
    }
    fn float_blue(&self) -> f64 {
        self.b as f64 / 255.0
    }

    /// This taken from [StackOverflow](https://stackoverflow.com/questions/596216/formula-to-determine-brightness-of-rgb-color)
    /// which referred to [Wikipedia's Relative Luminance](https://en.wikipedia.org/wiki/Luminance_%28relative%29).
    pub fn luminance(&self) -> f64 {
        self.float_red() * 0.2126 + self.float_green() * 0.7152 + self.float_blue() * 0.0722
    }
}

impl<'a> From<&'a (u8, u8, u8)> for Color {
    fn from(tuple: &'a (u8, u8, u8)) -> Color {
        Color::rgb(tuple.0, tuple.1, tuple.2)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct FixedSpriteData {
    pub data: Arc<Vec<Vec<Color>>>,
    // TODO: cache grayscale and rgba renders as Arc<Option<Vec<u8>>>?
}
impl FixedSpriteData {
    pub fn new(data: Vec<Vec<Color>>) -> FixedSpriteData {
        FixedSpriteData {
            data: Arc::new(data),
        }
    }
    pub fn width(&self) -> i32 {
        self.data[0].len() as i32
    }
    pub fn height(&self) -> i32 {
        self.data.len() as i32
    }
    pub fn make_black_version(&self) -> FixedSpriteData {
        let mut output = Vec::new();
        for pix_row in self.data.iter() {
            let mut row: Vec<Color> = Vec::new();
            for pixel in pix_row.iter() {
                if pixel.is_visible() {
                    row.push(Color::black());
                } else {
                    row.push(Color::invisible());
                }
            }
            output.push(row);
        }
        FixedSpriteData::new(output)
    }

    /// Given an include_bytes! png, convert it to a FixedSpriteData.
    pub fn load_png(data: &[u8]) -> FixedSpriteData {
        let decoder = png::Decoder::new(data);
        let (info, mut reader) = decoder.read_info().unwrap();
        let width = info.width as usize;
        let _height = info.height as usize;
        assert_eq!(info.color_type, png::ColorType::RGBA);
        assert_eq!(info.bit_depth, png::BitDepth::Eight);

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let mut output = Vec::new();
        for pix_row in buf.chunks(width * 4) {
            let mut row = Vec::new();
            for pix in pix_row.chunks(4) {
                row.push(Color::rgba(pix[0], pix[1], pix[2], pix[3]));
            }
            output.push(row);
        }

        FixedSpriteData::new(output)
    }

    pub fn find_visible_color(&self) -> Option<Color> {
        for row in self.data.iter() {
            for px in row {
                if px.is_visible() {
                    return Some(*px);
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteData {
    pub x: i32,
    pub y: i32,
    pub scale: i32,
    pub data: Vec<Vec<Color>>,
}
impl SpriteData {
    pub fn new(data: Vec<Vec<Color>>, scale: i32) -> SpriteData {
        SpriteData {
            x: 0,
            y: 0,
            scale,
            data,
        }
    }
    pub fn width(&self) -> i32 {
        self.data[0].len() as i32
    }
    pub fn height(&self) -> i32 {
        self.data.len() as i32
    }
    pub fn scale(&self) -> i32 {
        self.scale
    }
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn find_visible_color(&self) -> Option<Color> {
        for row in &self.data {
            for px in row {
                if px.is_visible() {
                    return Some(*px);
                }
            }
        }
        None
    }
    /// Make a full copy of this sprite with a new position.
    pub fn translate(&self, x: i32, y: i32) -> SpriteData {
        SpriteData {
            x,
            y,
            scale: self.scale,
            data: self.data.clone(),
        }
    }
}

#[derive(Clone)]
pub enum Drawable {
    Rectangle {
        color: Color,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    },
    /// For space_invaders.
    DestructibleSprite(SpriteData),
    /// For static images.
    StaticSprite {
        x: i32,
        y: i32,
        data: FixedSpriteData,
    },
}

impl Drawable {
    pub fn rect(color: Color, x: i32, y: i32, w: i32, h: i32) -> Drawable {
        Drawable::Rectangle { color, x, y, w, h }
    }
    pub fn sprite(x: i32, y: i32, sprite: FixedSpriteData) -> Drawable {
        Drawable::StaticSprite { x, y, data: sprite }
    }
}

pub struct GrayscaleBuffer {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}
impl GrayscaleBuffer {
    pub fn alloc(width: i32, height: i32) -> GrayscaleBuffer {
        GrayscaleBuffer {
            width,
            height,
            data: vec![0; (width * height) as usize],
        }
    }
    #[inline(always)]
    fn set_pixel(&mut self, x: i32, y: i32, color: u8) {
        let start = (y * self.width) + x;
        if start < 0 {
            return;
        }
        let start = start as usize;
        if start >= self.data.len() {
            return;
        }
        self.data[start] = color;
    }
    #[inline(always)]
    fn set_pixel_alpha(&mut self, x: i32, y: i32, color: Color) {
        if color.is_visible() {
            self.set_pixel(x, y, color.grayscale_byte())
        }
    }

    pub fn render(&mut self, commands: &[Drawable]) {
        for cmd in commands {
            match cmd {
                &Drawable::Rectangle { color, x, y, w, h } => {
                    let fill = color.grayscale_byte();
                    for yi in y..(y + h) {
                        for xi in x..(x + w) {
                            self.set_pixel(xi, yi, fill)
                        }
                    }
                }
                &Drawable::DestructibleSprite(ref sprite) => {
                    let w = sprite.width();
                    let h = sprite.height();
                    let (x, y) = sprite.position();
                    let scale = sprite.scale();
                    debug_assert!(scale > 0);
                    for yi in 0..h {
                        for xi in 0..w {
                            let color = sprite.data[yi as usize][xi as usize];
                            for xt in 0..sprite.scale {
                                for yt in 0..sprite.scale {
                                    self.set_pixel_alpha(xi + x + xt, yi + y + yt, color)
                                }
                            }
                        }
                    }
                }
                &Drawable::StaticSprite {
                    x,
                    y,
                    data: ref sprite,
                } => {
                    let w = sprite.width();
                    let h = sprite.height();
                    for yi in 0..h {
                        for xi in 0..w {
                            let color = sprite.data[yi as usize][xi as usize];
                            self.set_pixel_alpha(xi + x, yi + y, color)
                        }
                    }
                }
            }
        }
        // Done.
    }
}

pub struct ImageBuffer {
    pub width: i32,
    pub height: i32,
    /// Pixels encoded as RGBA.
    pub data: Vec<u8>,
}
impl ImageBuffer {
    pub fn alloc(width: i32, height: i32) -> ImageBuffer {
        ImageBuffer {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    #[inline(always)]
    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        debug_assert!(color.is_visible());
        let start = (y * self.width * 4) + (x * 4);
        if start < 0 {
            return;
        }
        let start = start as usize;
        if start >= self.data.len() {
            return;
        }
        self.data[start] = color.r;
        self.data[start + 1] = color.g;
        self.data[start + 2] = color.b;
        self.data[start + 3] = color.a;
    }

    #[inline(always)]
    fn set_pixel_alpha(&mut self, x: i32, y: i32, color: Color) {
        if color.is_visible() {
            self.set_pixel(x, y, color)
        }
    }

    pub fn render_sprite(&mut self, scale: i32, data: &Vec<Vec<Color>>) {
        let h = data.len() as i32;
        let w = data[0].len() as i32;
        for yi in 0..h {
            for xi in 0..w {
                let color = data[yi as usize][xi as usize];
                for xt in 0..scale {
                    for yt in 0..scale {
                        self.set_pixel_alpha(xi * scale + xt, yi * scale + yt, color)
                    }
                }
            }
        }
    }

    pub fn render(&mut self, commands: &[Drawable]) {
        for cmd in commands {
            match cmd {
                &Drawable::Rectangle { color, x, y, w, h } => {
                    for yi in y..(y + h) {
                        for xi in x..(x + w) {
                            self.set_pixel(xi, yi, color)
                        }
                    }
                }
                &Drawable::StaticSprite {
                    x,
                    y,
                    data: ref sprite,
                } => {
                    let w = sprite.width();
                    let h = sprite.height();
                    for yi in 0..h {
                        for xi in 0..w {
                            let color = sprite.data[yi as usize][xi as usize];
                            self.set_pixel_alpha(xi + x, yi + y, color)
                        }
                    }
                }
                &Drawable::DestructibleSprite(ref sprite) => {
                    let w = sprite.width();
                    let h = sprite.height();
                    let (x, y) = sprite.position();
                    let scale = sprite.scale();
                    debug_assert!(scale > 0);
                    for yi in 0..h {
                        for xi in 0..w {
                            let color = sprite.data[yi as usize][xi as usize];
                            for xt in 0..sprite.scale {
                                for yt in 0..sprite.scale {
                                    self.set_pixel_alpha(xi + x + xt, yi + y + yt, color)
                                }
                            }
                        }
                    }
                }
            }
        }
        // Done.
    }
}

/// Maybe deprecated? I made it OOP.
pub fn render_to_buffer(target: &mut ImageBuffer, commands: &[Drawable]) {
    target.render(commands);
}
