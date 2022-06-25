use bitflags::bitflags;

use crate::bit;


// Events in Hazel Oxide are currently blocking, meaning when an event occurs it
// immediately gets dispatched and must be dealt with right then and there.
// For the future, a better strategy might be to buffer events in an event
// bus and processing them during the "event" part of the update stage.

#[derive(PartialEq, Eq)]
pub enum EventType {
    None,
    WindowClosed, WindowResized, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

bitflags! {
    pub struct EventCategory: u32 {
        const NONE                        = 0;
        const EVENT_CATEGORY_APPLICATION  = bit!(0);
        const EVENT_CATEGORY_INPUT        = bit!(1);
        const EVENT_CATEGORY_KEYBOARD     = bit!(2);
        const EVENT_CATEGORY_MOUSE        = bit!(3);
        const EVENT_CATEGORY_MOUSE_BUTTON = bit!(4);
    }
}

pub struct Event {

    pub event_type:     EventType,
    pub name:           String,
    pub category_flags: u32,

    pub(crate) handled: bool,
}

impl Event {
    pub fn new(event_type: EventType, name: &str, category_flags: u32) -> Event {
        Event {
            event_type,
            name: name.to_string(),
            category_flags,

            handled: false,
        }
    }
}


pub trait TEvent {
    fn get_static_type() -> EventType;
    
    fn get_name(&self) -> String {
        String::from("")
    }

    fn get_event(& self) -> Event;
    
    fn get_event_type(&self) -> EventType;
    
    fn get_category_flags(&self) -> u32;
    
    fn is_in_category(&self, category: EventCategory) -> bool {
        (self.get_category_flags() & category.bits) == 0
    }
}


impl ToString for Event {
    fn to_string(&self) -> String {
        String::from(&self.name)
    }
}