use crate::{Message, CropMode, Draw, PagesState};
use iced::{Element, Alignment, Length, theme};
use iced::widget::{button, row, text, column, text_input, container, vertical_slider, Container};
use image::{RgbaImage};
use iced::widget::image as img;
use crate::SCREENSHOT_CONTAINER;

pub fn modify(screen_result: Option<RgbaImage>, draw: Draw, draw_text: String, screen_result_backup: Option<RgbaImage>, color_slider_value: u8, crop: CropMode) -> Element<'static, Message> {
    let home_btn = button(text("← Home").width(Length::Fill).size(20)).style(theme::Button::Destructive).on_press(Message::UpdatePage(PagesState::Home));

    let crop_btn = button(text(if crop == CropMode::CropStatus { "Crop" } else { "Confirm" }).width(Length::Fill).size(20))
        .style(if (draw == Draw::Crop && crop == CropMode::CropStatus) || (draw == Draw::Crop && crop == CropMode::CropConfirm) { theme::Button::Primary } else { theme::Button::Secondary })
        .on_press(Message::UpdateDraw(Draw::Crop));

    let free_draw_btn = button(text("Line").width(Length::Fill).size(20))
        .style(if draw == Draw::FreeHand { theme::Button::Positive } else { theme::Button::Secondary })
        .on_press(Message::UpdateDraw(Draw::FreeHand));

    let circle_draw_btn = button(text("Circle").width(Length::Fill).size(20))
        .style(if draw == Draw::Circle { theme::Button::Positive } else { theme::Button::Secondary })
        .on_press(Message::UpdateDraw(Draw::Circle));

    let arrow_draw_btn = button(text("Arrow").width(Length::Fill).size(20))
        .style(if draw == Draw::Arrow { theme::Button::Positive } else { theme::Button::Secondary })
        .on_press(Message::UpdateDraw(Draw::Arrow));

    let text_draw_btn = button(text("Text").width(Length::Fill).size(20))
        .style(if draw == Draw::Text { theme::Button::Positive } else { theme::Button::Secondary })
        .on_press(Message::UpdateDraw(Draw::Text));

    let text_draw_text = text_input("Enter text and place it in the image", draw_text.as_str()).width(Length::Fixed(350.0)).size(20)
        .on_input(|text| Message::UpdateDraw(Draw::TextInput(text)));

    let clear_btn = button(text("Clear").width(Length::Fill).size(20)).style(theme::Button::Destructive)
        .on_press(Message::UpdateDraw(Draw::ClearButton));

    let save_changes_btn = button(text("Save Changes").width(Length::Fill).size(20)).style(theme::Button::Positive)
        .on_press(Message::UpdateDraw(Draw::SaveModifyChanges));

    let control_row: Element<'static, Message>;

    if draw == Draw::Text && screen_result == screen_result_backup {
        control_row = row![home_btn, crop_btn, free_draw_btn, circle_draw_btn, arrow_draw_btn, text_draw_btn, text_draw_text].spacing(20).into();
    } else if draw == Draw::Text && screen_result != screen_result_backup {
        control_row = row![home_btn, crop_btn, free_draw_btn, circle_draw_btn, arrow_draw_btn, text_draw_btn, text_draw_text, clear_btn,save_changes_btn].spacing(20).into();
    } else if draw != Draw::Text && screen_result != screen_result_backup && crop == CropMode::CropStatus {
        control_row = row![home_btn, crop_btn, free_draw_btn, circle_draw_btn, arrow_draw_btn, text_draw_btn, clear_btn, save_changes_btn].spacing(20).into();
    } else if draw != Draw::Text && screen_result != screen_result_backup && crop == CropMode::CropConfirm {
        control_row = row![home_btn, crop_btn, free_draw_btn, circle_draw_btn, arrow_draw_btn, text_draw_btn, clear_btn].spacing(20).into();
    } else {
        control_row = row![home_btn, crop_btn, free_draw_btn, circle_draw_btn, arrow_draw_btn, text_draw_btn].spacing(20).into();
    }

    let image_row: Element<'static, Message>;

    match screen_result {
        Some(screen) => {
            let image = container(
                img(img::Handle::from_pixels(
                    screen.width(),
                    screen.height(),
                    screen.as_raw().clone(),
                ))
            )
                .id(SCREENSHOT_CONTAINER.clone())
                .style(theme::Container::Box);

            let image_container = container(image).height(Length::Fill).width(Length::Fill).center_y().center_x();

            let color_container = match color_slider_value {
                0..=9 => Container::new(row![]).style(style::black_container),
                10..=19 => Container::new(row![]).style(style::red_container),
                20..=29 => Container::new(row![]).style(style::orange_container),
                30..=39 => Container::new(row![]).style(style::yellow_container),
                40..=49 => Container::new(row![]).style(style::green_container),
                50..=59 => Container::new(row![]).style(style::blue_container),
                60..=69 => Container::new(row![]).style(style::indigo_container),
                70..=79 => Container::new(row![]).style(style::violet_container),
                _ => Container::new(row![]).style(style::white_container),
            }
                .height(50)
                .width(50);

            let slider = vertical_slider(0..=100, color_slider_value.clone(), |value| Message::UpdateDraw(Draw::ColorSlider(value))).step(1);
            let color_selector = column![text("Color Selector").size(20), color_container ].width(Length::Shrink).spacing(20);

            image_row = row![ image_container, slider, color_selector ].spacing(20).into();
        }
        None => {
            image_row = row![text("Error! No Screenshot")].spacing(20).align_items(Alignment::Center).into();
        }
    }

    let content: Element<_> = column![ control_row, image_row ].spacing(20).into();

    return container(content).height(Length::Fill).width(Length::Fill).into();
}

mod style {
    use iced::widget::container;
    use iced::{BorderRadius, Color, Theme};

    pub fn white_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 2.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn red_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn orange_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 165.0 / 255.0, 0.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn yellow_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 51.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn green_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn blue_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn indigo_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(73.0 / 255.0, 0.0 / 255.0, 130.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn violet_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(218.0 / 255.0, 112.0 / 255.0, 238.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn black_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}