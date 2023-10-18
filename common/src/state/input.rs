#[derive(Default)]
pub struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub right: bool,
    pub left: bool,
    pub space_toggle: bool,
    pub grab_mouse: bool,
}
