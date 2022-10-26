use glow::*;
use image::{io::Reader, DynamicImage, ImageError};

use super::model::Model;

#[derive(Clone, Copy)]
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

    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
    gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

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

pub struct CubeMap {
    id: Texture,
}

impl CubeMap {
    pub fn new(gl: &Context, path: &[&str]) -> Self {
        unsafe {
            let id = gl.create_texture().expect("Couldn't create texture.");
            gl.bind_texture(TEXTURE_CUBE_MAP, Some(id));

            for (i, p) in path.iter().enumerate() {
                let image =
                    load_image(*p).unwrap_or_else(|_| panic!("Couldn't load the image: {}", *p));
                let width = image.width();
                let height = image.height();

                let channel_count = image.color().channel_count();
                let format = match channel_count {
                    1 => RED,
                    3 => RGB,
                    4 => RGBA,
                    _ => RGBA,
                };

                gl.tex_image_2d(
                    TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    format as i32,
                    width as i32,
                    height as i32,
                    0,
                    format,
                    UNSIGNED_BYTE,
                    Some(image.as_bytes()),
                );
            }

            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_R, CLAMP_TO_EDGE as i32);

            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MIN_FILTER, LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MAG_FILTER, LINEAR as i32);

            Self { id }
        }
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.active_texture(TEXTURE0);
            gl.bind_texture(TEXTURE_CUBE_MAP, Some(self.id));
        }
    }
}

pub struct Skybox {
    cube_map: CubeMap,
    model: Model,
}

impl Skybox {
    pub fn new(gl: &Context, model: Model) -> Self {
        // let images = [
        //     "assets/skybox/right.jpg",
        //     "assets/skybox/left.jpg",
        //     "assets/skybox/top.jpg",
        //     "assets/skybox/bottom.jpg",
        //     "assets/skybox/front.jpg",
        //     "assets/skybox/back.jpg",
        // ];
        let images = [
            "assets/skybox2/right.bmp",
            "assets/skybox2/left.bmp",
            "assets/skybox2/top.bmp",
            "assets/skybox2/bottom.bmp",
            "assets/skybox2/front.bmp",
            "assets/skybox2/back.bmp",
        ];
        // let images = [
        //     "assets/skybox3/right.png",
        //     "assets/skybox3/left.png",
        //     "assets/skybox3/top.png",
        //     "assets/skybox3/bottom.png",
        //     "assets/skybox3/front.png",
        //     "assets/skybox3/back.png",
        // ];

        let cube_map = CubeMap::new(gl, &images);

        Self { cube_map, model }
    }

    pub fn cube_map(&self) -> &CubeMap {
        &self.cube_map
    }

    pub fn model(&self) -> &Model {
        &self.model
    }
}
