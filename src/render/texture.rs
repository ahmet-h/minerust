use glow::*;
use image::{io::Reader, DynamicImage, GenericImageView, ImageError, Pixel};

pub struct GameTexture {
    id: Texture,
}

impl GameTexture {
    pub fn new(gl: &Context, path: &str) -> Self {
        let image = load_image(path)
            .unwrap_or_else(|_| panic!("Couldn't load the image: {}", path))
            .flipv();
        let width = image.width();
        let height = image.height();

        let channel_count = image.color().channel_count();
        let format = match channel_count {
            1 => RED,
            3 => RGB,
            4 => RGBA,
            _ => RGBA,
        };

        let id = unsafe { create_native_texture(gl, width, height, format, image.as_bytes()) };

        Self { id }
    }

    pub fn bind(&self, gl: &Context, unit_index: i32) {
        unsafe {
            gl.active_texture(TEXTURE0 + unit_index as u32);
            gl.bind_texture(TEXTURE_2D, Some(self.id));
        }
    }
}

unsafe fn create_native_texture(
    gl: &Context,
    width: u32,
    height: u32,
    format: u32,
    pixels: &[u8],
) -> Texture {
    let id = gl.create_texture().expect("Couldn't create texture.");

    gl.bind_texture(TEXTURE_2D, Some(id));

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST_MIPMAP_LINEAR as i32);
    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAX_LEVEL, 4);

    gl.tex_image_2d(
        TEXTURE_2D,
        0,
        format as i32,
        width as i32,
        height as i32,
        0,
        format,
        UNSIGNED_BYTE,
        Some(pixels),
    );

    gl.generate_mipmap(TEXTURE_2D);

    gl.bind_texture(TEXTURE_2D, None);

    id
}

fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    let image = Reader::open(path)?.decode()?;

    Ok(image)
}
