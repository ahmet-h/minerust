use glow::Context;
use render::renderer::Renderer;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    video::{GLContext, GLProfile, Window},
    Sdl,
};
use state::{screen::Screen, AppState};

pub mod render;
pub mod state;

pub struct App {
    config: AppConfig,
    sdl: Sdl,
    window: Window,
    state: AppState,
}

impl App {
    pub fn run(&mut self) {
        let timer = self.sdl.timer().unwrap();
        let mut ticks = timer.performance_counter();
        let mut prev_ticks = ticks;
        let mut fps = 0;
        let mut t = 0;
        let mut acc: u64 = 0;
        let performance_freq = timer.performance_frequency();
        let fixed_timestep: u64 = performance_freq / 40;

        let mut mouse = self.sdl.mouse();
        let mut grab_mouse = false;

        // let sleep_duration = Duration::new(0, 1_000_000_000u32 / 1000);
        let mut event_pump = self.sdl.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                // let screen = game_state.screen_mut();
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    // Event::Window {
                    //     win_event: WindowEvent::FocusLost,
                    //     ..
                    // } => {
                    //     grab_mouse = false;
                    // }
                    // Event::Window {
                    //     win_event: WindowEvent::FocusGained,
                    //     ..
                    // } => {
                    //     grab_mouse = true;
                    // }
                    Event::Window {
                        win_event: WindowEvent::SizeChanged(w, h),
                        ..
                    } => {
                        println!("Window Resized: ({}, {})", w, h);
                        self.state.handle_resize(&self.window);
                    }
                    _ => {
                        self.state.handle_input(event);
                    }
                }
            }

            if grab_mouse != self.state.is_mouse_grabbed() {
                grab_mouse = self.state.is_mouse_grabbed();
                mouse.set_relative_mouse_mode(grab_mouse);
                mouse.show_cursor(!grab_mouse);

                mouse.warp_mouse_in_window(
                    &self.window,
                    self.config.window_width as i32 / 2,
                    self.config.window_height as i32 / 2,
                );
            }

            ticks = timer.performance_counter();
            let delta = ticks - prev_ticks;
            acc += delta;

            t += delta;
            if t >= performance_freq {
                t -= performance_freq;
                println!("FPS: {}", fps);
                fps = 0;
            }
            fps += 1;

            prev_ticks = ticks;

            while acc >= fixed_timestep {
                // save prev state

                acc -= fixed_timestep;

                // physics_update(&mut game_state);
            }

            // update(&mut game_state, delta as f32 / performance_freq as f32);

            let alpha = acc as f32 / fixed_timestep as f32;

            // render(&mut game_state, alpha);

            self.window.gl_swap_window();

            // std::thread::sleep(sleep_duration);
        }
    }
}

pub struct AppBuilder {
    config: AppConfig,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    pub fn build(&self) -> App {
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

        let state = AppState::new(gl, gl_context, &window);

        App {
            config: self.config.clone(),
            sdl: sdl_context,
            window,
            state,
        }
    }

    pub fn window_title(&mut self, window_title: &str) -> &mut Self {
        self.config.window_title = window_title.to_owned();
        self
    }

    pub fn window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.window_width = width;
        self.config.window_height = height;
        self
    }

    pub fn gl_context_version(&mut self, major: u8, minor: u8) -> &mut Self {
        self.config.gl_context_version_major = major;
        self.config.gl_context_version_minor = minor;
        self
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
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
