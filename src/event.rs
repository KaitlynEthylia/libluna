#[cfg(target_os = "windows")]
use winapi; //TODO once rust analyzer starts working

#[cfg(target_os = "linux")]
use evdev::InputEvent as EvEvent;

pub enum MouseBtn {
    LMB,
    RMB,
    MMB,
    MB4,
    MB5,
}

pub enum MouseScroll {
    Vertical,
    Horizontal,
}

pub enum MouseEvent {
    Btn {button: MouseBtn, down: bool},
    Move {x: i32, y: i32},
    Scroll {direction: MouseScroll, amount: i32},
}

pub enum KeyboardKey {
    Scancode(i32),
    Character(char),
}

pub enum KeyboardState {
    Up,
    Down, 
    Repeat,
}

#[non_exhaustive]
pub enum InputEvent {
    KeyboardEvent {key: KeyboardKey, state: KeyboardState},
    MouseEvent (MouseEvent),
    #[cfg(target_os = "linux")] LinEvdevEvent(EvEvent),
    #[cfg(target_os = "windows")] WinRawInputEvent(), //TODO once rust analyzer starts working
}
