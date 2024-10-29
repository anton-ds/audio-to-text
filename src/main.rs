mod delegate;
mod layout;
mod state;
mod transcriber;

use state::AppState;
use druid::{AppLauncher, WindowDesc};

fn main() {
    AppLauncher::with_window(build_window())
        .log_to_console()
        .delegate(delegate::Delegate)
        .launch(AppState::default())
        .expect("Failed to launch application");
}

fn build_window() -> WindowDesc<AppState> {
    WindowDesc::new(layout::main())
        .title("Transcriber")
        .window_size((500.0, 650.0))
}
