use crate::AppState;
use crate::transcriber::Transcriber;

use druid::{commands, Command, FileDialogOptions};
use druid::{AppDelegate, DelegateCtx, Env, EventCtx, Handled, Selector, Target};
use tokio::runtime::Runtime;

// Hooks for handling and modifying top-level events.

pub const UPDATE_TRANSCRIBE_RESULT: Selector<String> = Selector::new("app.update-transcribe-result");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {

    /// The AppDelegates Command handler.
    fn command(&mut self, _ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled {
        // Handle command `open file`
        if let Some(selected_file) = cmd.get(commands::OPEN_FILE) {
            data.selected_file = selected_file.path().to_string_lossy().into_owned();
            return Handled::Yes;
        }
        // Sets result into the results area
        if let Some(transcribe_result) = cmd.get(UPDATE_TRANSCRIBE_RESULT) {
            data.transcribe_result = transcribe_result.clone();
            return Handled::Yes;
        }

        Handled::No
    }
}

impl Delegate {

    /// Handles click on `Select file` button.
    /// Selected file will hook by `Delegate.command()`.
    pub fn on_select_file(ctx: &mut EventCtx) {
        ctx.submit_command(Command::new(
            commands::SHOW_OPEN_PANEL,
            FileDialogOptions::new(),
            Target::Auto,
        ));
    }

    /// Handles click on submit button.
    pub fn on_result_click(ctx: &mut EventCtx, data: &mut AppState) {

        // todo: should implement the spinner during processing
        //data.is_processing = true;

        let rt = Runtime::new().unwrap();
        let result = rt.block_on(async {
            let transcriber = Transcriber::new();
            transcriber.transcribe_file(&data.selected_file).await
        });

        match result {
            Ok(transcription) => Self::update_result(ctx, &transcription),
            Err(err) => Self::update_result(ctx, &err),
        }
    }

    /// Sends command to update the text in the results area
    fn update_result(ctx: &mut EventCtx, result: &str) {
        ctx.submit_command(
            Command::new(UPDATE_TRANSCRIBE_RESULT, result.to_string(), Target::Auto)
        );
        ctx.request_paint();
    }
}
