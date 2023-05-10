use bevy::prelude::*;
use iced::{Color, Element, Rectangle, Theme, Point, Padding, Size};
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

    let canvas: Element<UiMessage> = Canvas::new(Card { border_radius: 12. })
        .width(240)
        .height(320)
        .into();

    let column = widget::column(vec![
        text,
        btn,
        canvas,
        widget::text("after").into(),
    ]).padding(Padding::new(10.));

    ctx.display(column)
}

#[derive(Debug)]
struct Card {
    border_radius: f32,
}

impl Program<UiMessage> for Card {
    type State = ();

    fn draw(&self, _state: &(), _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        let mut frame = Frame::new(bounds.size());

        let points = vec![
            Point::new(0., bounds.size().height / 2.),
            Point::new(bounds.size().width / 2., 0.),
            Point::new(bounds.size().width, bounds.size().height / 2.),
            Point::new(bounds.size().width / 2., bounds.size().height),
        ];

        let corners = vec![
            Point::new(0., 0.),
            Point::new(bounds.width, 0.),
            Point::new(bounds.width, bounds.height),
            Point::new(0., bounds.height),
        ];

        let border = Path::new(|path| {
            path.move_to(points[0]);
            path.arc_to(corners[0], points[1], self.border_radius);
            path.line_to(points[1]);
            path.arc_to(corners[1], points[2], self.border_radius);
            path.line_to(points[2]);
            path.arc_to(corners[2], points[3], self.border_radius);
            path.line_to(points[3]);
            path.arc_to(corners[3], points[0], self.border_radius);
            path.line_to(points[0]);
            path.close()
        });
        frame.fill(&border, Color::WHITE);

        let art = Path::rectangle(
            Point::new(self.border_radius, self.border_radius),
            Size::new(bounds.width - (self.border_radius * 2.), bounds.height / 2.),
        );
        frame.fill(&art, Color::BLACK);

        vec![frame.into_geometry()]
    }
}
