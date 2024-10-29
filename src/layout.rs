use crate::state::AppState;
use crate::delegate::Delegate;

use druid::{Widget, WidgetExt};
use druid::widget::{Button, Either, Flex, Label, Spinner, TextBox};

pub fn main() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Label::new("Audio file:")
                .align_left()
                .padding(10.0)
        )
        .with_child(
            Button::new("Select file")
                .align_left()
                .padding(10.0)
                .on_click(|ctx, _, _| Delegate::on_select_file(ctx))
        )
        .with_child(
            Label::new(|data: &AppState, _: &_| data.selected_file.clone())
                .align_left()
                .padding(10.0)
        )
        .with_child(
            Label::new("Transcribe result:")
                .align_left()
                .padding(10.0)
        )
        .with_child(
            TextBox::multiline()
                .expand_width()
                .fix_height(370.0)
                .lens(AppState::transcribe_result)
                .padding(10.0)
        )
        .with_child(
            Button::new("GO!")
                .padding(10.0)
                .on_click(|ctx, data: &mut AppState, _| {
                    Delegate::on_result_click(ctx, data);
                })
        )
        .with_child(
            Either::new(
                |data: &AppState, _| data.is_processing,
                Spinner::new().padding(10.0),
                Label::new("").padding(10.0),
            )
        )
}