use bevy::prelude::*;
use iced::{Color, Element, Rectangle, Theme};
use iced::widget;
use iced::widget::canvas::*;
use bevy_iced::{IcedContext, IcedPlugin};

#[derive(Debug, Clone)]
pub enum UiMessage {}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(IcedPlugin)
        .add_event::<UiMessage>()
        .add_system(bevy::window::close_on_esc)
        .add_system(ui_system)
        .run();
}

fn ui_system(time: Res<Time>, mut ctx: IcedContext<UiMessage>) {

    let text: Element<UiMessage> = widget::text(format!(
            "Hello Iced! Running for {:.2} seconds.",
            time.elapsed_seconds(),
        ))
        .into();

    let btn: Element<UiMessage> = widget::button("Button!")
        .into();

    let canvas: Element<UiMessage> = Canvas::new(Circle { radius: 50.0 })
        .into();

    let column = widget::column(vec![
        text,
        btn,
        canvas,
        widget::text("after").into(),
    ]);

    ctx.display(column)
}

#[derive(Debug)]
struct Circle {
    radius: f32,
}

impl Program<UiMessage> for Circle {
    type State = ();

    fn draw(&self, _state: &(), _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(bounds.size());
        let circle = Path::circle(frame.center(), self.radius);
        frame.fill(&circle, Color::WHITE);
        vec![frame.into_geometry()]
    }
}
