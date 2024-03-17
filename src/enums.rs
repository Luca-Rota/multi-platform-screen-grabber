use crate::choice::Choice;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagesState {
    Home,
    Settings,
    Modify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionState {
    Screenshotting,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Draw {
    FreeHand,
    Circle,
    Text,
    Arrow,
    Nothing,
    Crop,
    TextInput(String),
    SaveModifyChanges,
    ClearButton,
    ColorSlider(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Setting {
    Monitor(Choice),
    Format(Choice),
    Autosave(bool),
    Clipboard(bool),
    Shortcut(bool),
    Path,
    Timer(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropMode {
    CropStatus,
    CropConfirm,
}

