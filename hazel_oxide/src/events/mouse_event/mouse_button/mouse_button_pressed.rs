use crate::events::mouse_event::mouse_button::*;

struct MouseButtonPressedEvent {
    pub button_event: MouseButtonEvent,

    //TODO: event class type
}

impl MouseButtonPressedEvent {
    pub fn new(button: i32) -> MouseButtonPressedEvent {

        MouseButtonPressedEvent {
            button_event: MouseButtonEvent::new(EventType::MouseButtonPressed, "Mouse Button Pressed", button),
        }
    }
}

impl ToString for MouseButtonPressedEvent {
    fn to_string(&self) -> String {

        format!("MousePressedEvent: {}", self.button_event.button)
    }
}
