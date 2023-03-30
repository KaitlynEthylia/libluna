use crate::event::{InputEvent, MouseEvent, MouseBtn, MouseScroll, KeyboardState};
use evdev::InputEvent as EvEvent;
use evdev::{EventType, Key};

impl InputEvent {
    fn to_evdev_event(&self) -> Option<Vec<EvEvent>> {
        let mut vec = match self {
            InputEvent::LinEvdevEvent(event) => vec!(event.clone()),
            InputEvent::MouseEvent(event) => {
                match event {
                    MouseEvent::Move { x, y } => vec!(
                        EvEvent::new(EventType::RELATIVE, 0, *x),
                        EvEvent::new(EventType::RELATIVE, 1, *y)),
                    MouseEvent::Scroll { direction, amount } => {
                        match direction {
                            MouseScroll::Horizontal => vec!(
                                EvEvent::new(EventType::RELATIVE, 6, amount.signum()),
                                EvEvent::new(EventType::RELATIVE, 12, *amount)),
                            MouseScroll::Vertical => vec!(
                                EvEvent::new(EventType::RELATIVE, 8, amount.signum()),
                                EvEvent::new(EventType::RELATIVE, 11, *amount)),
                        }
                    },
                    MouseEvent::Btn { button, down } => {
                        let (key, scancode) = match button {
                            MouseBtn::LMB => (Key::BTN_LEFT, 589825),
                            MouseBtn::RMB => (Key::BTN_RIGHT, 589826),
                            MouseBtn::MMB => (Key::BTN_MIDDLE, 589827),
                            MouseBtn::MB4 => (Key::BTN_SIDE, 589828),
                            MouseBtn::MB5 => (Key::BTN_EXTRA, 589829),
                        };
                        
                        vec!(EvEvent::new(EventType::KEY, key.0, *down as i32),
                            EvEvent::new(EventType::MISC, 4, scancode))
                    },
                }
            },
            InputEvent::KeyboardEvent { key, state } => {
                let value = match state {
                    KeyboardState::Up => 0,
                    KeyboardState::Down => 1,
                    KeyboardState::Repeat => 2,
                };
                let key: u16;
                let scancode: i32;

                vec!(EvEvent::new(EventType::KEY, todo!(), value),
                    EvEvent::new(EventType::MISC, 4, todo!()))
            },
            _ => vec!(),
        };

        match vec.len() {
            0 => None,
            _ => {
                vec.push(EvEvent::new(EventType::SYNCHRONIZATION, 0, 0));
                Some(vec)
            },
        }
    }
}
