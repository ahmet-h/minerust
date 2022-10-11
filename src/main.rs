use glow::*;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    video::GLProfile,
};

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

fn init_sdl() -> (
    sdl2::Sdl,
    sdl2::video::Window,
    glow::Context,
    sdl2::video::GLContext,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    // Enable MSAA anti-aliasing
    // gl_attr.set_multisample_buffers(1);
    // gl_attr.set_multisample_samples(8);

    let window = video_subsystem
        .window("Minerust", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _)
    };

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    (sdl_context, window, gl, gl_context)
}

fn main() {
    let (sdl_context, window, gl, _gl_context) = init_sdl();

    let timer = sdl_context.timer().unwrap();
    let mut ticks = timer.performance_counter();
    let mut prev_ticks = ticks;
    let mut fps = 0;
    let mut t = 0;
    let mut acc: u64 = 0;
    let performance_freq = timer.performance_frequency();
    let fixed_timestep: u64 = performance_freq / 40;

    let mouse = sdl_context.mouse();
    mouse.show_cursor(false);
    mouse.warp_mouse_in_window(&window, WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    mouse.set_relative_mouse_mode(true);
    let mut grab_mouse = true;

    unsafe {
        // Enable MSAA anti-aliasing
        // gl::Enable(gl::MULTISAMPLE);
        gl.enable(glow::DEPTH_TEST);
        gl.enable(glow::CULL_FACE);
        gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        gl.viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

        gl.clear_color(0.5, 0.9, 1.0, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    window.gl_swap_window();

    // let sleep_duration = Duration::new(0, 1_000_000_000u32 / 60);
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
                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } => {
                    grab_mouse = false;
                }
                Event::Window {
                    win_event: WindowEvent::FocusGained,
                    ..
                } => {
                    grab_mouse = true;
                }
                _ => {
                    if grab_mouse {
                        // screen.handle_input(event);
                    }
                }
            }
        }

        if grab_mouse {
            // Fix for relative mouse mode not working properly on Windows with custom dpi settings
            // mouse.warp_mouse_in_window(window, WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
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

            // physics_update(game_state);
        }

        // update(game_state, delta as f32 / performance_freq as f32);

        let alpha = acc as f32 / fixed_timestep as f32;

        // render(game_state, alpha);
        unsafe {
            gl.clear_color(0.5, 0.9, 1.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        window.gl_swap_window();

        // std::thread::sleep(sleep_duration);
    }
}
