
use super::event::*;

// TODO: https://www.youtube.com/watch?v=xnopUoZbMEk&list=PLlrATfBNZ98dC-V-N3m0Go4deliWHPFwT&index=12
// timestamp 28:12
pub struct EventDispatcher<T: TEvent> {
    event: T,
}


impl<T: TEvent + ToString> EventDispatcher<T> {

    pub fn dispatch(
        &self, 
        func: impl Fn(&T) -> bool
    ) -> bool {
    
        let matched = self.event.get_event_type() == T::get_static_type();

        if matched {
            self.event.get_event().handled = func(&self.event);
        }

        matched
    }

    pub fn get_event(&self) -> &T {
        &self.event
    }
}

impl<T: TEvent + ToString> ToString for EventDispatcher<T> {
    fn to_string(&self) -> String {
        self.event.to_string()
    }
}
