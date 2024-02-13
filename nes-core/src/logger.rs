use std::cell::Cell;

thread_local! {
    pub static PRINT_PTR: Cell<fn(&str) -> ()> = Cell::new(print_default);
}


#[macro_export]
macro_rules! log {
    ($($arg:tt),*) => {
        let text = format!($($arg),*);
        $crate::logger::PRINT_PTR.get()(&text);
    };
}

pub fn register_logger(fun: fn(&str) -> ()) {
    PRINT_PTR.set(fun);
}

fn print_default(text: &str) {
    println!("{}", &text);
}