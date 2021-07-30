use flex_error::{define_error, DisplayOnly, TraceError};

define_error! {
    Error{
        Dummy
            |_| { format_args!("dummy error") },
    }
}
