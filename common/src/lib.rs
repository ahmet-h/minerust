use glow::Context;
use render::renderer::Renderer;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    video::{GLContext, GLProfile, Window},
    Sdl,
};
use state::screen::Screen;

pub mod render;
pub mod state;

pub struct App {
    config: AppConfig,
    sdl: Option<Sdl>,
    window: Option<Window>,
    state: Option<AppState>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App::default();

        app.init();

        app
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
        self.state = Some(AppState::new(gl, gl_context, &window));
        self.window = Some(window);
    }

    pub fn run(&mut self) {
        let sdl_context = self.sdl.as_ref().unwrap();
        let window = self.window.as_ref().unwrap();
        let state = self.state.as_mut().unwrap();

        let timer = sdl_context.timer().unwrap();
        let mut ticks = timer.performance_counter();
        let mut prev_ticks = ticks;
        let mut fps = 0;
        let mut t = 0;
        let mut acc: u64 = 0;
        let performance_freq = timer.performance_frequency();
        let fixed_timestep: u64 = performance_freq / 40;

        let mut mouse = sdl_context.mouse();
        let mut grab_mouse = false;

        // let sleep_duration = Duration::new(0, 1_000_000_000u32 / 1000);
        let mut event_pump = sdl_context.event_pump().unwrap();
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
                        // game_state.handle_resize(&window);
                    }
                    _ => {
                        if grab_mouse {
                            // game_state.handle_input(event);
                        }

                        state.handle_input(event);
                    }
                }
            }

            if grab_mouse != state.is_mouse_grabbed() {
                grab_mouse = state.is_mouse_grabbed();
                mouse.set_relative_mouse_mode(grab_mouse);
                mouse.show_cursor(!grab_mouse);

                mouse.warp_mouse_in_window(
                    window,
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

            window.gl_swap_window();

            // std::thread::sleep(sleep_duration);
        }
    }
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

pub struct AppState {
    renderer: Renderer,
    screens: Vec<Screen>,
}

impl AppState {
    pub fn new(gl: Context, gl_context: GLContext, window: &Window) -> Self {
        let renderer = Renderer::new(gl, gl_context, window);
        let screen = Screen::new(&renderer);

        let screens = vec![screen];

        Self { renderer, screens }
    }

    pub fn handle_input(&mut self, event: Event) {
        if let Some(screen) = self.screens.last_mut() {
            screen.handle_input(event);
        }
    }

    pub fn is_mouse_grabbed(&self) -> bool {
        if let Some(screen) = self.screens.last() {
            return screen.is_mouse_grabbed();
        }

        false
    }
}
