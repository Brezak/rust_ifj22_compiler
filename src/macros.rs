#[macro_export]
macro_rules! warn {
    ($span:expr, $message:expr) => {
        chumsky::error::Simple::custom($span, format!("Warn:{}", $message))
    };
}

#[macro_export]
macro_rules! error {
    ($span:expr, $message:expr) => {
        chumsky::error::Simple::custom($span, format!("Error:{}", $message))
    };
}

#[macro_export]
macro_rules! unreachable {
    ($span:expr) => {
        chumsky::error::Simple::custom(
            $span,
            format!(
                "Critical:reached unreachable code at [{}:{}:{}]",
                core::file!(),
                core::line!(),
                core::column!()
            ),
        )
    };

    ($span:expr, $message:expr) => {
        chumsky::error::Simple::custom(
            $span,
            format!(
                "Critical:reached unreachable code at [{}:{}:{}] with message: {}",
                core::file!(),
                core::line!(),
                core::column!(),
                $message
            ),
        )
    };
}
