use edgy::{
    embedded_graphics::pixelcolor::BinaryColor,
    themes::{DynamicStyle, WidgetStyle},
};

pub const BUTTON_STYLE: DynamicStyle<BinaryColor> = DynamicStyle {
    active: WidgetStyle::new()
        .accent_color(BinaryColor::On)
        .background_color(BinaryColor::On)
        .foreground_color(BinaryColor::Off),
    drag: WidgetStyle::new()
        .accent_color(BinaryColor::On)
        .background_color(BinaryColor::On)
        .foreground_color(BinaryColor::Off),
    focus: WidgetStyle::new()
        .accent_color(BinaryColor::On)
        .background_color(BinaryColor::On)
        .foreground_color(BinaryColor::Off),
    idle: WidgetStyle::new()
        .accent_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .foreground_color(BinaryColor::On),
};

pub const BATTERY_INDICATOR_STYLE: WidgetStyle<BinaryColor> = WidgetStyle::new()
    .background_color(BinaryColor::Off)
    .foreground_color(BinaryColor::On)
    .storke(1, BinaryColor::On)
    .accent_color(BinaryColor::On);
