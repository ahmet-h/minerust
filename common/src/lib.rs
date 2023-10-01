use glow::Context;
use sdl2::{
    video::{GLProfile, Window},
    Sdl,
};

pub mod render;

pub struct App {
    config: AppConfig,
    sdl: Option<Sdl>,
    window: Option<Window>,
    state: Option<AppState>,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    fn init(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(
            self.config.gl_context_version_major,
            self.config.gl_context_version_minor,
        );

        // Enable MSAA anti-aliasing
        // gl_attr.set_multisample_buffers(1);
        // gl_attr.set_multisample_samples(8);

        let window = video_subsystem
            .window(
                &self.config.window_title,
                self.config.window_width,
                self.config.window_height,
            )
            .opengl()
            // .fullscreen_desktop()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = unsafe {
            Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _)
        };
        // video_subsystem.gl_set_swap_interval(0).unwrap(); // disable vsync

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(
            gl_attr.context_version(),
            (
                self.config.gl_context_version_major,
                self.config.gl_context_version_minor
            )
        );

        self.sdl = Some(sdl_context);
        self.window = Some(window);
    }

    pub fn run(&self) {}
}

impl Default for App {
    fn default() -> Self {
        let config = AppConfig::default();

        Self {
            config,
            sdl: None,
            window: None,
            state: None,
        }
    }
}

pub struct AppConfig {
    window_title: String,
    window_width: u32,
    window_height: u32,
    gl_context_version_major: u8,
    gl_context_version_minor: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "App".to_owned(),
            window_width: 1280,
            window_height: 720,
            gl_context_version_major: 3,
            gl_context_version_minor: 3,
        }
    }
}

pub struct AppState {}
