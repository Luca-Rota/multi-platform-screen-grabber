pub mod ui {
    pub mod home;
    pub mod modify;
    pub mod settings;
}

use crate::ui::home::home;
use crate::ui::modify::modify;
use crate::ui::settings::settings;
use iced::{executor, mouse};
use iced::widget::{container};
use iced::window;
use iced::{Application, Command, Subscription, Element, Length, Settings, Theme, Size, Event, Rectangle};
use std::{thread};
use std::mem::replace;
use tokio::sync::mpsc;
use std::cell::RefCell;
use std::time::{Duration};
use iced::window::{UserAttention};
use multi_platform_screen_grabbing_utility::screenshot::Screenshot;
use multi_platform_screen_grabbing_utility::image_handler::ImageHandler;
use multi_platform_screen_grabbing_utility::hotkeys::{check_shortcut_event, generate_current_time_string};
use multi_platform_screen_grabbing_utility::choice::Choice;
use multi_platform_screen_grabbing_utility::enums::{Setting, PagesState, CropMode, SubscriptionState, Draw};
use rfd::FileDialog;
use once_cell::sync::Lazy;
use image::Rgba;
use image::{GenericImageView, RgbaImage, SubImage, imageops};
use crate::CropMode::CropStatus;
use rusttype::{Font, Scale};
use crate::Draw::{FreeHand, Nothing};
use imageproc::rect::Rect;
use arboard::Clipboard;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (350, 120),
            ..Default::default()
        },
        ..Settings::default()
    };
    return ScreenshotGrabber::run(settings);
}

#[derive(Debug, Clone)]
pub enum Message {
    NewScreenshotButton,
    ScreenDone(Vec<Option<RgbaImage>>),
    ChangeSelectedScreen(usize),
    SaveButton,
    EventOccurred(Event),
    ModifyImage(Option<Rectangle>, Option<Event>),
    UpdateDraw(Draw),
    UpdateSetting(Setting),
    UpdatePage(PagesState),
}

static SCREENSHOT_CONTAINER: Lazy<container::Id> = Lazy::new(|| container::Id::new("screenshot"));

#[derive()]
struct ScreenshotGrabber {
    page_state: PagesState,
    sender: RefCell<Option<mpsc::UnboundedSender<Vec<Option<RgbaImage>>>>>,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<Vec<Option<RgbaImage>>>>>,
    toggler_value_clipboard: bool,
    toggler_value_autosave: bool,
    radio_value_monitor: Choice,
    radio_value_format: Choice,
    timer_value: i32,
    shortcut_value: String,
    path_value: String,
    shortcut_listen: bool,
    screen_result: Vec<Option<RgbaImage>>,
    screen_selected: usize,
    subscription_state: SubscriptionState,
    total_monitor_number: usize,
    crop: CropMode,
    crop_start: (i32, i32),
    crop_end: (i32, i32),
    width: u32,
    height: u32,
    draw: Draw,
    draw_mouse_pressed: bool,
    draw_figure_press: (i32, i32),
    draw_figure_released: (i32, i32),
    draw_text_input: String,
    draw_color_slider_value: u8,
    image_to_modify: Option<RgbaImage>,
}

impl Application for ScreenshotGrabber {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (tx, rx) = mpsc::unbounded_channel::<Vec<Option<RgbaImage>>>();
        return (ScreenshotGrabber {
            page_state: PagesState::Home,
            sender: RefCell::new(Some(tx)),
            receiver: RefCell::new(Some(rx)),
            toggler_value_clipboard: false,
            toggler_value_autosave: false,
            radio_value_monitor: Choice::A,
            radio_value_format: Choice::A,
            timer_value: 0,
            shortcut_value: "CTRL + s".to_string(),
            shortcut_listen: false,
            path_value: "".to_string(),
            screen_result: Vec::new(),
            screen_selected: 0,
            subscription_state: SubscriptionState::None,
            total_monitor_number: Screenshot::monitors_num(),
            crop: CropStatus,
            crop_start: (0, 0),
            crop_end: (0, 0),
            width: 1,
            height: 1,
            draw: Nothing,
            draw_mouse_pressed: false,
            draw_figure_press: (0, 0),
            draw_figure_released: (0, 0),
            draw_text_input: "".to_string(),
            draw_color_slider_value: 0,
            image_to_modify: None,
        }, Command::none());
    }

    fn title(&self) -> String {
        String::from("Iced Screen Grabber utility")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        return match message {
            Message::UpdatePage(value) => {
                match value {
                    PagesState::Settings => {
                        self.page_state = PagesState::Settings;
                        return window::resize(Size::new(1000, 500));
                    }
                    PagesState::Home => {
                        self.page_state = PagesState::Home;
                        self.draw = Nothing;
                        if self.screen_result.is_empty() {
                            return window::resize(Size::new(350, 120));
                        } else {
                            return window::resize(Size::new(1000, 500));
                        }
                    }
                    PagesState::Modify => {
                        self.page_state = PagesState::Modify;
                        return window::maximize(true);
                    }
                }
            }
            Message::NewScreenshotButton => {
                let sender = self.sender.clone();
                let timer_value = self.timer_value.clone();
                let radio_value_monitor = self.radio_value_monitor.to_numeric().clone();
                self.subscription_state = SubscriptionState::Screenshotting;
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis((timer_value * 1000 + 500) as u64));
                    let mut screen_results = Vec::new();
                    if radio_value_monitor != 6 {
                        match Screenshot::capture_screen(radio_value_monitor - 1) {
                            Ok(res) => {
                                let img = res.convert().unwrap();
                                screen_results.push(Some(img));
                            }
                            Err(err) => {
                                screen_results.push(None);
                                eprintln!("Error: {}", err);
                            }
                        }
                    } else {
                        match Screenshot::capture_all() {
                            Ok(res) => {
                                for screen in res {
                                    let img = screen.convert().unwrap();
                                    screen_results.push(Some(img));
                                }
                            }
                            Err(err) => {
                                screen_results.push(None);
                                eprintln!("Error: {}", err);
                            }
                        }
                    }
                    sender.take().as_mut().unwrap().send(screen_results).unwrap();
                });
                window::minimize(true)
            }
            Message::SaveButton => {
                if !self.screen_result.is_empty() {
                    if let Some(img) = self.screen_result[self.screen_selected.clone()].clone() {
                        let current_time_string = generate_current_time_string();
                        let path = std::env::current_dir().unwrap();
                        let imghndl: ImageHandler = img.clone().into();
                        let res = FileDialog::new()
                            .set_file_name(current_time_string)
                            .set_directory(&path)
                            .add_filter("png", &["png"])
                            .add_filter("jpg", &["jpg"])
                            .add_filter("gif", &["gif"])
                            .save_file();
                        match res {
                            Some(save_path) => {
                                ImageHandler::save_image(&imghndl, save_path);
                            }
                            None => ()
                        }
                    }
                }
                Command::none()
            }
            Message::ChangeSelectedScreen(value) => {
                self.screen_selected = value;
                self.image_to_modify = self.screen_result[value.clone()].clone();
                Command::none()
            }
            Message::ScreenDone(images) => {
                self.screen_result = images.clone();
                self.screen_selected = 0;
                self.image_to_modify = self.screen_result[0].clone();
                self.subscription_state = SubscriptionState::None;
                let (tx, rx) = mpsc::unbounded_channel::<Vec<Option<RgbaImage>>>();
                self.sender = RefCell::new(Some(tx));
                self.receiver = RefCell::new(Some(rx));
                if self.toggler_value_clipboard {
                    match Clipboard::new() {
                        Ok(mut cb) => {
                            for screen in &self.screen_result {
                                match screen {
                                    Some(img) => {
                                        let img_clipboard: ImageHandler = img.clone().into();
                                        if let Err(err) = img_clipboard.to_clipboard(&mut cb) {
                                            eprintln!("Error copying image to clipboard: {:?}", err);
                                        }
                                    }
                                    None => (),
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating clipboard: {:?}", err);
                        }
                    }
                }
                if self.toggler_value_autosave {
                    for (index, screen) in self.screen_result.iter().enumerate() {
                        match screen {
                            Some(img) => {
                                let current_time_string = generate_current_time_string();
                                let filename = format!("{}{}_{}{}", self.path_value, current_time_string, index, self.radio_value_format.to_format());
                                let imghndl: ImageHandler = img.clone().into();
                                ImageHandler::save_image(&imghndl, filename.into());
                            }
                            None => ()
                        }
                    }
                }
                window::request_user_attention(Some(UserAttention::Informational))
            }
            Message::UpdateSetting(value) => {
                match value {
                    Setting::Monitor(value) => { self.radio_value_monitor = value; }
                    Setting::Format(value) => { self.radio_value_format = value; }
                    Setting::Autosave(value) => { self.toggler_value_autosave = value; }
                    Setting::Clipboard(value) => { self.toggler_value_clipboard = value; }
                    Setting::Shortcut(value) => { self.shortcut_listen = value; }
                    Setting::Timer(value) => { self.timer_value = value; }
                    Setting::Path => {
                        let res = FileDialog::new().pick_folder();
                        match res {
                            Some(path) => {
                                self.path_value = format!("{}\\", path.display());
                            }
                            None => ()
                        }
                    }
                }
                return Command::none();
            }
            Message::UpdateDraw(value) => {
                match value {
                    Draw::Crop => {
                        if self.draw == Draw::Crop && self.crop == CropMode::CropStatus {
                            self.draw = Draw::Nothing;
                        } else if self.draw == Draw::Crop && self.crop == CropMode::CropConfirm {
                            //Da in alto a sinistra a destra
                            if self.crop_end.0.clone() - self.crop_start.0.clone() > 0 && self.crop_end.1.clone() - self.crop_start.1.clone() > 0 {
                                let cropped: SubImage<&RgbaImage> = self.image_to_modify.as_ref().unwrap().view((self.crop_start.0.clone() + 1) as u32, (self.crop_start.1.clone() + 1) as u32, (self.width.clone() - 2) as u32, (self.height.clone() - 2) as u32);
                                let cropped_image = cropped.to_image();
                                let resized_image = imageops::resize(&cropped_image, 1920, 1080, image::imageops::FilterType::Lanczos3);
                                self.image_to_modify = Some(resized_image);
                            }
                            //Da in alto a destra a sinistra
                            if self.crop_end.0.clone() - self.crop_start.0.clone() <= 0 && self.crop_end.1.clone() - self.crop_start.1.clone() > 0 {
                                let cropped: SubImage<&RgbaImage> = self.image_to_modify.as_ref().unwrap().view((self.crop_end.0.clone() + 1) as u32, (self.crop_start.1.clone() + 1) as u32, (self.width.clone() - 2) as u32, (self.height.clone() - 2) as u32);
                                let cropped_image = cropped.to_image();
                                let resized_image = imageops::resize(&cropped_image, 1920, 1080, image::imageops::FilterType::Lanczos3);
                                self.image_to_modify = Some(resized_image);
                            }
                            //Da in basso a destra a sinistra
                            if self.crop_end.0.clone() - self.crop_start.0.clone() <= 0 && self.crop_end.1.clone() - self.crop_start.1.clone() <= 0 {
                                let cropped: SubImage<&RgbaImage> = self.image_to_modify.as_ref().unwrap().view((self.crop_end.0.clone() + 1) as u32, (self.crop_end.1.clone() + 1) as u32, (self.width.clone() - 2) as u32, (self.height.clone() - 2) as u32);
                                let cropped_image = cropped.to_image();
                                let resized_image = imageops::resize(&cropped_image, 1920, 1080, image::imageops::FilterType::Lanczos3);
                                self.image_to_modify = Some(resized_image);
                            }
                            //Da in basso a sinistra a destra
                            if self.crop_end.0.clone() - self.crop_start.0.clone() > 0 && self.crop_end.1.clone() - self.crop_start.1.clone() <= 0 {
                                let cropped: SubImage<&RgbaImage> = self.image_to_modify.as_ref().unwrap().view((self.crop_start.0.clone() + 1) as u32, (self.crop_end.1.clone() + 1) as u32, (self.width.clone() - 2) as u32, (self.height.clone() - 2) as u32);
                                let cropped_image = cropped.to_image();
                                let resized_image = imageops::resize(&cropped_image, 1920, 1080, image::imageops::FilterType::Lanczos3);
                                self.image_to_modify = Some(resized_image);
                            }
                            self.crop = CropMode::CropStatus;
                            self.draw = Draw::Nothing;
                            self.crop_start = (0, 0);
                            self.crop_end = (0, 0);
                        } else {
                            self.draw = Draw::Crop;
                        }
                    }
                    Draw::Arrow if self.crop != CropMode::CropConfirm => { if self.draw == Draw::Arrow { self.draw = Nothing; } else { self.draw = Draw::Arrow; } }
                    Draw::FreeHand if self.crop != CropMode::CropConfirm => { if self.draw == FreeHand { self.draw = Nothing; } else { self.draw = FreeHand; } }
                    Draw::Circle if self.crop != CropMode::CropConfirm => { if self.draw == Draw::Circle { self.draw = Nothing; } else { self.draw = Draw::Circle; } }
                    Draw::TextInput(value) => { self.draw_text_input = value; }
                    Draw::ColorSlider(value) => { self.draw_color_slider_value = value; }
                    Draw::Text if self.crop != CropMode::CropConfirm => {
                        if self.draw == Draw::Text {
                            self.draw = Nothing;
                            self.draw_text_input = "".to_string();
                        } else { self.draw = Draw::Text; }
                    }
                    Draw::ClearButton => {
                        self.image_to_modify = self.screen_result[self.screen_selected.clone()].clone();
                        self.crop = CropStatus;
                        self.crop_start = (0, 0);
                        self.crop_end = (0, 0);
                    }
                    Draw::SaveModifyChanges if self.crop != CropMode::CropConfirm => {
                        for (index, elem) in self.screen_result.iter_mut().enumerate() {
                            if index == self.screen_selected {
                                let _ = replace(elem, self.image_to_modify.clone());
                            }
                        }
                    }
                    _ => ()
                }
                Command::none()
            }
            Message::ModifyImage(screenshot_bounds, event) => {
                let color;
                match self.draw_color_slider_value.clone() {
                    0..=9 => { color = Rgba([0u8, 0u8, 0u8, 255u8]); }
                    10..=19 => { color = Rgba([255u8, 0u8, 0u8, 255u8]); }
                    20..=29 => { color = Rgba([255u8, 165u8, 0u8, 255u8]); }
                    30..=39 => { color = Rgba([255u8, 255u8, 51u8, 255u8]); }
                    40..=49 => { color = Rgba([34u8, 139u8, 34u8, 255u8]); }
                    50..=59 => { color = Rgba([0u8, 0u8, 255u8, 255u8]); }
                    60..=69 => { color = Rgba([73u8, 0u8, 130u8, 255u8]); }
                    70..=79 => { color = Rgba([218u8, 112u8, 238u8, 255u8]); }
                    _ => { color = Rgba([255u8, 255u8, 255u8, 255u8]); }
                }
                let mut window_size = (0.0, 0.0);
                if screenshot_bounds.clone().unwrap().width != 0.0 && screenshot_bounds.clone().unwrap().height != 0.0 {
                    window_size = ((self.image_to_modify.clone().unwrap().width() as f32) / screenshot_bounds.clone().unwrap().width, (self.image_to_modify.clone().unwrap().height() as f32) / screenshot_bounds.clone().unwrap().height);
                }
                match self.draw {
                    FreeHand if self.crop != CropMode::CropConfirm => {
                        let screen = self.image_to_modify.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() {
                                    let position = (((position.x.clone() - screenshot_bounds.unwrap().x.clone()) * window_size.clone().0) as i32, ((position.y.clone() - screenshot_bounds.unwrap().y.clone()) * window_size.clone().1) as i32);
                                    self.image_to_modify = Some(imageproc::drawing::draw_filled_circle(&screen, position, 5, color.clone()));
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                            }
                            _ => {}
                        };
                    }
                    Draw::Circle if self.crop != CropMode::CropConfirm => {
                        let screen = self.image_to_modify.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press == (0, 0) {
                                    self.draw_figure_press = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone().0) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y.clone()) * window_size.clone().1) as i32);
                                }
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press != (0, 0) {
                                    self.draw_figure_released = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone().0) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y) * window_size.clone().1) as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                                self.image_to_modify = Some(imageproc::drawing::draw_hollow_circle(&screen, self.draw_figure_press.clone(), (((self.draw_figure_released.0.clone() - self.draw_figure_press.0.clone()).pow(2) + (self.draw_figure_released.1.clone() - self.draw_figure_press.1.clone()).pow(2)) as f64).sqrt() as i32, color.clone()));
                                self.draw_figure_press = (0, 0);
                                self.draw_figure_released = (0, 0);
                            }
                            _ => {}
                        };
                    }
                    Draw::Text if self.crop != CropMode::CropConfirm => {
                        let screen = self.image_to_modify.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) {
                                    self.draw_figure_press = ((position.clone().x - screenshot_bounds.clone().unwrap().x) as i32, (position.clone().y - screenshot_bounds.clone().unwrap().y) as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.image_to_modify = Some(imageproc::drawing::draw_text(&screen, color.clone(), (self.draw_figure_press.0.clone() as f32 * window_size.clone().0) as i32, (self.draw_figure_press.1.clone() as f32 * window_size.clone().1) as i32, Scale { x: 24.8, y: 24.8 }, &Font::try_from_vec(Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8])).unwrap(), self.draw_text_input.clone().as_str()));
                                self.draw_figure_press = (0, 0);
                            }
                            _ => {}
                        };
                    }
                    Draw::Arrow if self.crop != CropMode::CropConfirm => {
                        let screen = self.image_to_modify.clone().unwrap();
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press == (0, 0) {
                                    self.draw_figure_press = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone().0) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y.clone()) * window_size.clone().0) as i32);
                                }
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.draw_figure_press != (0, 0) {
                                    self.draw_figure_released = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone().0) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y) * window_size.clone().0) as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                                let mut slope = (self.draw_figure_released.clone().1 - self.draw_figure_press.clone().1) as f32 / (self.draw_figure_released.clone().0 - self.draw_figure_press.clone().0) as f32;
                                if slope > 0.0 && slope <= 0.5 { slope = 0.5; } else if slope > 1.0 { slope = 1.0; } else if slope < 0.0 && slope >= -0.5 { slope = -0.5; } else if slope < -1.0 { slope = -1.0; }
                                if self.draw_figure_press.1 > self.draw_figure_released.1 {
                                    let image_tmp1 = imageproc::drawing::draw_line_segment(&screen, ((self.draw_figure_released.clone().0 as f32 + (30.0 * slope.clone())), (self.draw_figure_released.clone().1 as f32 + (30.0 * slope.clone()))), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color.clone());
                                    let image_tmp2 = imageproc::drawing::draw_line_segment(&image_tmp1, ((self.draw_figure_released.clone().0 as f32 + (30.0 * slope.clone())), (self.draw_figure_released.clone().1 as f32 - (30.0 * slope.clone()))), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color.clone());
                                    self.image_to_modify = Some(imageproc::drawing::draw_line_segment(&image_tmp2, (self.draw_figure_press.clone().0 as f32, self.draw_figure_press.clone().1 as f32), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color.clone()));
                                } else {
                                    let image_tmp1 = imageproc::drawing::draw_line_segment(&screen, ((self.draw_figure_released.clone().0 as f32 - (30.0 * slope.clone())), (self.draw_figure_released.clone().1 as f32 - (30.0 * slope.clone()))), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color.clone());
                                    let image_tmp2 = imageproc::drawing::draw_line_segment(&image_tmp1, ((self.draw_figure_released.clone().0 as f32 - (30.0 * slope.clone())), (self.draw_figure_released.clone().1 as f32 + (30.0 * slope.clone()))), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color.clone());
                                    self.image_to_modify = Some(imageproc::drawing::draw_line_segment(&image_tmp2, (self.draw_figure_press.clone().0 as f32, self.draw_figure_press.clone().1 as f32), (self.draw_figure_released.clone().0 as f32, self.draw_figure_released.clone().1 as f32), color.clone()));
                                }
                                self.draw_figure_press = (0, 0);
                                self.draw_figure_released = (0, 0);
                            }
                            _ => {}
                        };
                    }
                    Draw::Crop => {
                        let screen = self.image_to_modify.clone().unwrap();
                        let color = Rgba([255u8, 0u8, 0u8, 255u8]);
                        let mut rect = Rect::at(1, 1).of_size(1, 1);
                        match event {
                            Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                                //println!("{} {}",position.x,position.y);
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.crop_start == (0, 0) {
                                    self.crop_start = (((position.x.clone() - screenshot_bounds.unwrap().x.clone()) * window_size.clone().0) as i32, ((position.y.clone() - screenshot_bounds.unwrap().y.clone()) * window_size.clone().1) as i32);
                                }
                                if screenshot_bounds.unwrap().contains(position) && self.draw_mouse_pressed.clone() && self.crop_start != (0, 0) {
                                    self.crop_end = (((position.x.clone() - screenshot_bounds.unwrap().x.clone()) * window_size.clone().0) as i32, ((position.y.clone() - screenshot_bounds.unwrap().y.clone()) * window_size.clone().1) as i32);
                                }
                            }
                            Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = true;
                            }
                            Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                                self.draw_mouse_pressed = false;
                                //println!("x1:{} y1:{} x2:{} y3:{}",self.crop_start.0,self.crop_start.1,self.crop_end.0,self.crop_end.1);
                                if self.crop != CropMode::CropConfirm {
                                    //Da in alto a sinistra a destra
                                    if self.crop_end.0.clone() - self.crop_start.0.clone() > 0 && self.crop_end.1.clone() - self.crop_start.1.clone() > 0 {
                                        self.width = (self.crop_end.0.clone() - self.crop_start.0.clone()) as u32;
                                        self.height = (self.crop_end.1.clone() - self.crop_start.1.clone()) as u32;
                                        if self.width > 0 && self.height > 0 {
                                            rect = Rect::at(self.crop_start.0.clone(), self.crop_start.1.clone()).of_size(self.width, self.height);
                                        }
                                    }
                                    //Da in alto a destra a sinistra
                                    if self.crop_end.0.clone() - self.crop_start.0.clone() <= 0 && self.crop_end.1.clone() - self.crop_start.1.clone() > 0 {
                                        self.width = (self.crop_start.0.clone() - self.crop_end.0.clone()) as u32;
                                        self.height = (self.crop_end.1.clone() - self.crop_start.1.clone()) as u32;
                                        if self.width > 0 && self.height > 0 {
                                            rect = Rect::at(self.crop_end.0.clone(), self.crop_start.1.clone()).of_size(self.width, self.height);
                                        }
                                    }
                                    //Da in basso a destra a sinistra
                                    if self.crop_end.0.clone() - self.crop_start.0.clone() <= 0 && self.crop_end.1.clone() - self.crop_start.1.clone() <= 0 {
                                        self.width = (self.crop_start.0.clone() - self.crop_end.0.clone()) as u32;
                                        self.height = (self.crop_start.1.clone() - self.crop_end.1.clone()) as u32;
                                        if self.width > 0 && self.height > 0 {
                                            rect = Rect::at(self.crop_end.0.clone(), self.crop_end.1.clone()).of_size(self.width, self.height);
                                        }
                                    }
                                    //Da in basso a sinistra a destra
                                    if self.crop_end.0.clone() - self.crop_start.0.clone() > 0 && self.crop_end.1.clone() - self.crop_start.1.clone() <= 0 {
                                        self.width = (self.crop_end.0.clone() - self.crop_start.0.clone()) as u32;
                                        self.height = (self.crop_start.1.clone() - self.crop_end.1.clone()) as u32;
                                        if self.width > 0 && self.height > 0 {
                                            rect = Rect::at(self.crop_start.0.clone(), self.crop_end.1.clone()).of_size(self.width, self.height);
                                        }
                                    }
                                    if self.width > 0 && self.height > 0 {
                                        self.image_to_modify = Some(imageproc::drawing::draw_hollow_rect(&screen, rect, color));
                                    }
                                }
                                if self.width > 0 && self.height > 0 {
                                    self.crop = CropMode::CropConfirm;
                                }
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                }

                return Command::none();
            }
            Message::EventOccurred(event) => {
                if check_shortcut_event(&event) == self.shortcut_value {
                    return Command::perform(async { Message::NewScreenshotButton }, |msg| msg);
                }
                if self.shortcut_listen {
                    if check_shortcut_event(&event) != "".to_string() {
                        self.shortcut_value = check_shortcut_event(&event);
                        self.shortcut_listen = false;
                    }
                }
                if !self.screen_result.is_empty() {
                    if self.screen_result[self.screen_selected.clone()].is_some() && event == Event::Window(window::Event::Focused) {
                        return window::resize(Size::new(1000, 500));
                    }
                }
                if self.page_state == PagesState::Modify {
                    return container::visible_bounds(SCREENSHOT_CONTAINER.clone()).map(move |bounds| { Message::ModifyImage(bounds, Some(event.clone())) });
                }
                Command::none()
            }
        };
    }

    fn view(&self) -> Element<Message> {
        return container(
            match self.page_state {
                PagesState::Home => home(self.screen_result.clone(), self.screen_selected.clone(), self.toggler_value_autosave.clone()),
                PagesState::Settings => settings(self.toggler_value_autosave.clone(), self.toggler_value_clipboard.clone(), self.radio_value_monitor, self.radio_value_format, self.timer_value.clone(), self.shortcut_value.clone(), self.path_value.clone(), self.total_monitor_number.clone(), self.shortcut_listen.clone()),
                PagesState::Modify => modify(self.image_to_modify.clone(), self.draw.clone(), self.draw_text_input.clone(), self.screen_result[self.screen_selected.clone()].clone(), self.draw_color_slider_value.clone(), self.crop.clone()),
            })
            .width(Length::Fill)
            .padding(25)
            .center_x()
            .center_y()
            .into();
    }

    fn subscription(&self) -> Subscription<Message> {
        return match self.subscription_state {
            SubscriptionState::Screenshotting => {
                iced::subscription::unfold(
                    "channel",
                    self.receiver.take(),
                    move |mut receiver| async move {
                        let mut images = Vec::new();
                        while images.is_empty() {
                            match receiver.as_mut().unwrap().recv().await {
                                Some(imgs) => {
                                    for screen in imgs {
                                        images.push(screen);
                                    }
                                }
                                None => ()
                            };
                        }
                        return (Message::ScreenDone(images), receiver);
                    },
                )
            }
            SubscriptionState::None => {
                iced::subscription::events().map(Message::EventOccurred)
            }
        };
    }
}