#[macro_export]
macro_rules! alert {
    ($msg:expr) => {
        $crate::popups::alert_message($msg, "Alert", None, None, None);
    };
    ($msg:expr, $title:expr) => {
        $crate::popups::alert_message($msg, $title, None, None, None);
    };
    ($msg:expr, $title:expr, $buttons:expr) => {
        $crate::popups::alert_message($msg, $title, Some($buttons), None, None);
    };
    ($msg:expr, $title:expr, $buttons:expr, $yes_callback:expr) => {
        $crate::popups::alert_message($msg, $title, Some($buttons), Some($yes_callback), None);
    };
    ($msg:expr, $title:expr, $buttons:expr, $yes_callback:expr, $no_callback:expr) => {
        $crate::popups::alert_message(
            $msg,
            $title,
            Some($buttons),
            Some($yes_callback),
            Some($no_callback),
        );
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::popups::error_message($msg, "Error", None);
    };
    ($msg:expr, $title:expr) => {
        $crate::popups::error_message($msg, $title, None);
    };
    ($msg:expr, $title:expr, $callback:expr) => {
        $crate::popups::error_message($msg, $title, Some($callback));
    };
}
