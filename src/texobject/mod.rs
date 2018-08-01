extern crate sdl2;
extern crate png;


use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use self::png::{Decoder, ColorType, DecodingError};

use std::io::Read;

use scene::Drawable;

pub struct PngImage {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl PngImage {
    pub fn load_from_path<R: Read>(r: R) -> Result<Self, DecodingError> {
        let (info, mut reader) = Decoder::new(r).read_info()?;

        let (width, height) = (info.width as usize, info.height as usize);

        let mut data = vec![0; width * height * 4];

        for y in 0..height {
            if let Some(row) = reader.next_row()? {
                assert_eq!(row.len(), width * info.color_type.samples());

                for (x, col) in row.chunks(info.color_type.samples()).enumerate() {

                    let sdl_col = match info.color_type {
                        ColorType::RGB => { Color::RGB(col[0], col[1], col[2]) },
                        ColorType::RGBA => { Color::RGBA(col[0], col[1], col[2], col[3]) },
                        _ => { unimplemented!() }
                    };

                    data[(y * width + x) * 4 + 0] = sdl_col.a;
                    data[(y * width + x) * 4 + 1] = sdl_col.b;
                    data[(y * width + x) * 4 + 2] = sdl_col.g;
                    data[(y * width + x) * 4 + 3] = sdl_col.r;
                }
            }
        }

        Ok(PngImage {
            width:  width,
            height: height,
            data:   data,
        })
    }

}

impl Drawable for PngImage {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let creator = canvas.texture_creator();
        let mut texture = creator
                .create_texture_target(
                    None,
                    self.height as u32,
                    self.width  as u32).expect("Can't make texture");


        texture.update(None, self.data.as_slice(), 4 * self.width);

        canvas.copy(&texture, None, Rect::new(0, 0, self.width as u32, self.height as u32));

    }
}
