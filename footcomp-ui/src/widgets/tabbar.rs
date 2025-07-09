use edgy::{
    embedded_graphics::{
        mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_4X6},
        pixelcolor::BinaryColor,
        prelude::*,
        primitives::{PrimitiveStyle, Rectangle},
        text::{Alignment, Baseline, Text, TextStyleBuilder, renderer::TextRenderer},
    },
    prelude::*,
};

pub struct TabBarLabel<'a> {
    active: bool,
    text: &'a str,
    style: MonoTextStyle<'a, BinaryColor>,
}

impl<'a> TabBarLabel<'a> {
    pub fn new(active: bool, text: &'a str) -> Self {
        Self {
            active,
            text,
            style: MonoTextStyleBuilder::new()
                .font(&FONT_4X6)
                .text_color(BinaryColor::On)
                .build(),
        }
    }
}

impl<'a, D: DrawTarget<Color = BinaryColor>> Widget<'a, D, BinaryColor> for TabBarLabel<'a> {
    fn is_interactive(&mut self) -> bool {
        false
    }

    fn size(&mut self, _context: &mut UiContext<'a, D, BinaryColor>, _hint: Size) -> Size {
        let mut size = self
            .style
            .measure_string(self.text, Point::zero(), Baseline::Middle)
            .bounding_box
            .size;
        size.height += 1;
        size
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, BinaryColor>,
        rect: Rectangle,
        _event_args: WidgetEvent,
    ) -> EventResult {
        let text = Text::with_text_style(
            &self.text,
            rect.top_left,
            self.style,
            TextStyleBuilder::new()
                .alignment(Alignment::Left)
                .baseline(Baseline::Top)
                .build(),
        );

        let _ = text.draw(&mut context.draw_target);
        if self.active {
            let mut pos = rect.top_left;
            pos.y += rect.size.height as i32;
            let _ = Rectangle::new(pos, Size::new(rect.size.width - 1, 1))
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(&mut context.draw_target);
        }

        EventResult::Pass
    }
}
