use iced::{Element, Length, alignment, theme};
use iced::widget::{button, row, text, column, container, Row};
use crate::{Message, Choice, Setting, PagesState};
use iced::widget::{horizontal_space, scrollable, toggler, vertical_space, Radio, Container};

fn shortcut_input(shortcut_value: String, shortcut_listen: bool) -> Container<'static, Message> {
    let text_box;
    if shortcut_listen {
        text_box = Container::new(text("Press Ctrl/Shift/Alt + A-Z").size(16).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
            .style(style::text_container)
            .height(30)
            .width(220)
            .padding([0, 0, 0, 20]);
    } else {
        text_box = Container::new(text(shortcut_value.clone()).size(20).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
            .style(style::text_container)
            .height(30)
            .width(160)
            .padding([0, 0, 0, 20]);
    }
    let select_button = button("Change shortcut").on_press(Message::UpdateSetting(Setting::Shortcut(true)));
    let setting_input = row![text_box, select_button ].spacing(20);
    let container = Container::new(setting_input);
    return container;
}

fn path_input(path_value: String) -> Container<'static, Message> {
    let path_len = path_value.len();
    let text_size = if path_len > 50 { 11 } else if path_len > 47 { 12 } else if path_len > 44 { 13 } else if path_len > 41 { 14 } else if path_len > 48 { 15 } else if path_len > 35 { 16 } else if path_len > 32 { 17 } else { 18 };
    let text_box = Container::new(text(path_value.clone()).size(text_size).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
        .style(style::text_container)
        .height(30)
        .width(350)
        .padding([5, 10]);
    let select_button = button("Change path").on_press(Message::UpdateSetting(Setting::Path));
    let setting_input = row![text_box, select_button ].spacing(20);
    let container = Container::new(setting_input);
    return container;
}

fn timer_container(timer_value: i32) -> Container<'static, Message> {
    let increment_button = button(text("+").width(Length::Fixed(25.0)).height(Length::Fixed(25.0)).size(25).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
        .on_press(if timer_value < 10 { Message::UpdateSetting(Setting::Timer(timer_value.clone() + 1 )) } else { Message::UpdateSetting(Setting::Timer(timer_value.clone())) });
    let timer_text = Container::new(text(timer_value.clone()).size(20)).padding([5, 20, 0, 20]);
    let decrement_button = button(text("-").width(Length::Fixed(25.0)).height(Length::Fixed(25.0)).size(25).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
        .on_press(if timer_value > 0 { Message::UpdateSetting(Setting::Timer(timer_value.clone() - 1 )) } else { Message::UpdateSetting(Setting::Timer(timer_value.clone())) });
    let setting_input = row![decrement_button, timer_text, increment_button];
    let container = Container::new(setting_input);
    return container;
}

fn radio_container_format(radio_value: Choice) -> Container<'static, Message> {
    let selected_choice = Some(radio_value);
    let a = Radio::new(".jpg", Choice::A, selected_choice, |b|Message::UpdateSetting(Setting::Format(b)));
    let container_a = Container::new(a).padding([0, 10]);
    let b = Radio::new(".png", Choice::B, selected_choice, |b|Message::UpdateSetting(Setting::Format(b)));
    let container_b = Container::new(b).padding([0, 10]);
    let c = Radio::new(".gif", Choice::C, selected_choice, |b|Message::UpdateSetting(Setting::Format(b)));
    let container_c = Container::new(c).padding([0, 10]);
    let setting_input = row![container_a, container_b, container_c];
    let container = Container::new(setting_input);
    return container;
}

fn radio_container_monitor(radio_value: Choice, total_monitor_number: usize) -> Container<'static, Message> {
    let selected_choice = Some(radio_value);
    let tmn = if total_monitor_number > 5 { 5 } else { total_monitor_number };
    let mut radio_row = Row::new().spacing(20);
    for i in 1..=tmn {
        let label = format!("{}", i);
        let value = match i {
            1 => Choice::A,
            2 => Choice::B,
            3 => Choice::C,
            4 => Choice::D,
            5 => Choice::E,
            _ => Choice::A,
        };
        radio_row = radio_row.push(Radio::new(&label, value, selected_choice, |b|Message::UpdateSetting(Setting::Monitor(b))));
    }
    radio_row = radio_row.push(Radio::new("All", Choice::F, selected_choice, |b|Message::UpdateSetting(Setting::Monitor(b))));
    let container = Container::new(radio_row);
    return container;
}

fn toggler_container(toggler_value: bool, toggler_type: String) -> Container<'static, Message> {
    let setting_input = toggler(
        String::from(""),
        toggler_value,
        move |b| { if toggler_type == "autosave" { Message::UpdateSetting(Setting::Autosave(b)) } else { Message::UpdateSetting(Setting::Clipboard(b)) } },
    )
        .width(Length::Shrink)
        .spacing(10);
    let container = Container::new(setting_input);
    return container;
}

fn settings_box(settings_text: String, settings_container: Container<'static, Message>) -> Container<'static, Message> {
    let settingtext = text(settings_text)
        .size(18)
        .vertical_alignment(alignment::Vertical::Center);
    let text_container = Container::new(settingtext).padding([0, 0, 0, 10]);
    let space = horizontal_space(Length::Fill);
    let setting = row![text_container, space, settings_container];
    let container = Container::new(setting)
        .style(style::settings_container)
        .height(80)
        .width(Length::Fill)
        .padding(10)
        .align_y(alignment::Vertical::Center);
    return container;
}

pub fn settings(toggler_value_autosave: bool, toggler_value_clipboard: bool, radio_value_monitor: Choice, radio_value_format: Choice, timer_value: i32, shortcut_value: String, path_value: String, total_monitor_number: usize, shortcut_listen: bool) -> Element<'static, Message> {
    let undobutton = button(text("← Home").width(Length::Fill).size(20))
        .on_press(Message::UpdatePage(PagesState::Home))
        .style(theme::Button::Destructive)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(50.0))
        .padding(10);

    let controls = row![undobutton];
    let spacev = vertical_space(Length::Fixed(20.0));

    let container1 = settings_box("Save the screenshot automatically".to_string(), toggler_container(toggler_value_autosave, "autosave".to_string()));
    let container2 = settings_box("Copy the screenshot into the clipdoard automatically".to_string(), toggler_container(toggler_value_clipboard, "clipboard".to_string()));
    let container3 = settings_box("Select the monitor in which to screenshot".to_string(), radio_container_monitor(radio_value_monitor, total_monitor_number));
    let container4 = settings_box("Set a shortcut to make the screenshots".to_string(), shortcut_input(shortcut_value, shortcut_listen));
    let container5 = settings_box("Select the screenshot format".to_string(), radio_container_format(radio_value_format));
    let container6 = settings_box("Set a timer before the screenshot".to_string(), timer_container(timer_value));
    let container7 = settings_box("Change the path where you save the screenshot".to_string(), path_input(path_value));

    let content: Element<_> = column![ controls, spacev, container1, container2, container3, container4, container5, container6, container7 ]
        .spacing(20)
        .padding(20)
        .into();

    let scrollable = scrollable(
        container(content)
            .width(Length::Fill)
            .center_x(),
    );

    return container(scrollable).height(Length::Fill).center_y().into();
}

mod style {
    use iced::widget::container;
    use iced::{BorderRadius, Color, Theme};

    pub fn settings_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(0.0 / 255.0, 203.0 / 255.0, 247.0 / 255.0).into()),
            border_radius: BorderRadius::from(15.0),
            border_width: 3.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }

    pub fn text_container(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();
        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(Color::from_rgb(87.0 / 255.0, 115.0 / 255.0, 240.0 / 255.0).into()),
            border_radius: BorderRadius::from(5.0),
            border_width: 2.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}