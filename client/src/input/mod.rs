
#[derive(Debug, PartialEq)]
pub enum KeyState {
    UP, DOWN,
}

#[derive(Debug)]
pub struct KeyboardEvent {
    pub state: KeyState,
    pub key: String,
}

pub struct ClickEvent {
    // ???
}

#[derive(Copy, Clone, Debug)]
pub struct MouseMoveEvent {
    pub dx: f64,
    pub dy: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct ResizeEvent {
    pub width: u32,
    pub height: u32,
}
