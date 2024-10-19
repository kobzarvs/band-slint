use crate::AppWindow;
use slint::Weak;

pub fn slint_invoke(
    arc_weak_ui_clone: Weak<AppWindow>,
    callback: impl FnOnce(AppWindow) + Send + 'static,
) {
    slint::invoke_from_event_loop(move || {
        if let Some(ui) = arc_weak_ui_clone.upgrade() {
            callback(ui);
        }
    })
    .expect("Failed to invoke callback from slint event loop");
}
