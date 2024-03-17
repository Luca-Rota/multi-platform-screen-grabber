use iced::{Element, Length, theme, Alignment, alignment};
use iced::widget::{button, row, text, container, column};
use crate::{Message,PagesState};
use iced::widget::image as img;
use image::RgbaImage;

pub fn home(screen_result: Vec<Option<RgbaImage>>, screen_selected: usize, toggler_value_autosave: bool) -> Element<'static, Message> {
    let control_row: Element<'static, Message>;
    let mut image_row: Element<'static, Message> = row![].into();

    let screen_btn = button(text("New Screenshot").width(Length::Fill).size(20)).style(theme::Button::Primary).on_press(Message::NewScreenshotButton);
    let settings_btn = button(text("Settings").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::UpdatePage(PagesState::Settings));
    let modify_btn = button(text("Modify").width(Length::Fill).size(20)).style(theme::Button::Secondary).on_press(Message::UpdatePage(PagesState::Modify));
    let save_btn = button(text("Save").width(Length::Fill).size(20)).style(theme::Button::Positive).on_press(Message::SaveButton);
    let left_btn = button(text("←").width(Length::Fixed(30.0)).height(Length::Fixed(30.0)).size(30).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center)).style(theme::Button::Primary)
        .on_press(if screen_selected > 0 { Message::ChangeSelectedScreen(screen_selected - 1) } else { Message::ChangeSelectedScreen(0) });
    let right_btn = button(text("→").width(Length::Fixed(30.0)).height(Length::Fixed(30.0)).size(30).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center)).style(theme::Button::Primary)
        .on_press(if screen_selected + 1 < screen_result.len() { Message::ChangeSelectedScreen(screen_selected + 1) } else { Message::ChangeSelectedScreen(screen_selected) });

    let screenshot = if screen_result.is_empty() { None } else { screen_result[screen_selected].clone() };

    match screenshot {
        Some(screen) => {
            if toggler_value_autosave {
                control_row = row![screen_btn, settings_btn, modify_btn ]
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into();
            } else {
                control_row = row![screen_btn, settings_btn, modify_btn, save_btn]
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into();
            }
            let image = img(img::Handle::from_pixels(
                screen.width(),
                screen.height(),
                screen.as_raw().clone(),
            ));

            let image_container = container(image).height(Length::Fill).width(Length::Fill).center_y().center_x();

            if screen_result.len() == 1 {
                image_row = row![image_container]
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into();
            } else {
                image_row = row![left_btn, image_container, right_btn]
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .into();
            }
        }
        None => {
            control_row = row![screen_btn, settings_btn]
                .spacing(20)
                .align_items(Alignment::Center)
                .into();
        }
    }

    let content: Element<_> = column![ control_row, image_row ]
        .spacing(20)
        .into();

    return container(content).height(Length::Fill).width(Length::Fill).into();
}