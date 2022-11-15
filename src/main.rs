use bevy::{
    app::AppExit,
    prelude::*,
    window::WindowResizeConstraints,
};
use bevy::prelude::{Transform};
use bevy_pixels::prelude::*;
use pixels::wgpu::Color;

use raqote::*;

use euclid::{point2, Point2D};

// use font_kit::loaders::default::Font;
use font_kit::source::SystemSource;
use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Weight};
// use std::io::Read;
// use std::fs::File;
// use std::sync::Arc;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const SCALE: f32 = 1.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Hello Bevy Pixels".to_string(),
                position: WindowPosition::At((0., 0.).into()),
                width: WIDTH as f32 * SCALE,
                height: HEIGHT as f32 * SCALE,
                resize_constraints: WindowResizeConstraints {
                    min_width: WIDTH as f32,
                    min_height: HEIGHT as f32,
                    ..default()
                },
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(PixelsPlugin {
            width: WIDTH,
            height: HEIGHT,
        })
        .add_startup_system(setup)
        .add_system_to_stage(PixelsStage::Draw, main_system)
        // .add_system(main_system)
        .add_system(exit_on_escape)
        .run();
}

#[derive(Component)]
struct Card {
    name: String,
    attack: u32,
    defence: u32,
}

struct Sprite {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,

}

impl Sprite {
    fn to_draw_target(&self) -> DrawTarget {
        DrawTarget::from_backing(self.width as i32, self.height as i32, self.data.clone())
    }

    fn to_box2d(&self) -> euclid::Box2D<i32, euclid::UnknownUnit> {
        euclid::rect(0 as i32, 0 as i32, self.width as i32, self.height as i32).to_box2d()
    }

    pub fn blit_to_buffer(&self, buffer: &mut [u8], dest: Point2D<i32, euclid::UnknownUnit>) {
        let u32_buffer = bytemuck::try_cast_slice_mut::<u8, u32>(buffer).unwrap();
        let mut dt = DrawTarget::from_backing(WIDTH as i32, HEIGHT as i32, u32_buffer);
        dt.copy_surface(&self.to_draw_target(), self.to_box2d(), dest);
    }
}

#[derive(Resource)]
struct Sprites {
}

impl Sprites {
    fn build_card_sprite(card: &Card) -> Sprite {
        let width = CARD_WIDTH as i32;
        let height = CARD_HEIGHT as i32;
        let spacing = 4;
        let mut dt = DrawTarget::new(width, height);
        let color = raqote::Color::new(0xff, 0xff, 0xff, 0xff);
        dt.fill_rect(0., 0., width as f32, height as f32, &Source::Solid(SolidSource::from(color)), &DrawOptions::new());

        let pic_color = &Source::Solid(SolidSource::from(raqote::Color::new(0xff, 0x33, 0x33, 0x33)));
        dt.fill_rect(4., 30., (width - 8) as f32, 80., pic_color, &DrawOptions::new());

        // let font = &Font::from_path("./assets/Passageway-8qzz.otf", 0).unwrap();
        let font = &SystemSource::new()
            .select_best_match(
                &[FamilyName::Title("Lucida Sans".into())],
                &Properties::new().weight(Weight::MEDIUM),
            ).unwrap().load().unwrap();

        let font_size = 24.;
        let text_color = Source::Solid(SolidSource::from(raqote::Color::new(0xff, 0x00, 0x00, 0x00)));
        dt.draw_text(font, font_size, card.name.as_str(), point2(spacing as f32, font_size), &text_color, &DrawOptions::new());
        dt.draw_text(font, font_size, format!("{}", card.attack).as_str(), point2(4., (height - spacing) as f32), &text_color, &DrawOptions::new());
        dt.draw_text(font, font_size, format!("{}", card.defence).as_str(), point2((width - 20) as f32, (height - spacing) as f32), &text_color, &DrawOptions::new());

        Sprite {
            width: width as u32,
            height: height as u32,
            data: dt.get_data().to_owned(),
        }
    }
}

const CARD_WIDTH: u32 = 156;
const CARD_HEIGHT: u32 = (CARD_WIDTH as f32 * 1.4) as u32;
const BORDER_SIZE: u32 = 2;

fn card_pos(index: u32) -> Transform {
    let x = (CARD_WIDTH * index) + (BORDER_SIZE * (index + 1));
    Transform::from_xyz(x as f32, BORDER_SIZE as f32, 0.)
}

fn setup(mut cmd: Commands) {
    cmd.spawn((
        card_pos(0),
        Card {
            name: "My Card".to_owned(),
            attack: 1,
            defence: 3,
        },
    ));
    cmd.spawn((
        card_pos(1),
        Card {
            name: "Other Card".to_owned(),
            attack: 2,
            defence: 2,
        },
    ));
    cmd.spawn((
        card_pos(2),
        Card {
            name: "Card 3".to_owned(),
            attack: 2,
            defence: 2,
        },
    ));
    cmd.spawn((
        card_pos(3),
        Card {
            name: "Card 4".to_owned(),
            attack: 2,
            defence: 2,
        },
    ));
    cmd.spawn((
        card_pos(4),
        Card {
            name: "Card 5".to_owned(),
            attack: 2,
            defence: 2,
        },
    ));
}

fn main_system(
    mut pixels_resource: ResMut<PixelsResource>,
    // windows: ResMut<Windows>,
    cards: Query<(&Card, &Transform)>,
) {
    pixels_resource.pixels.set_clear_color(Color::BLACK);
    let frame: &mut [u8] = pixels_resource.pixels.get_frame_mut();


    for (card, t) in cards.iter()  {
        let sprite = Sprites::build_card_sprite(card);
        sprite.blit_to_buffer(frame, point2(
            t.translation.x as i32,
            t.translation.y as i32,
        ));
    }

    // let main_window = windows.get_primary().unwrap();
    // pixels_resource.pixels.resize_surface(main_window.physical_width() as u32, main_window.physical_height() as u32);
    // pixels_resource.pixels.resize_surface(1600 as u32, 1200 as u32);
    // println!("{:?}", (main_window.physical_width(), main_window.physical_height()));
    // println!("{:?}", pixels_resource.pixels.context().scaling_renderer.clip_rect());

    // let sprite = &sprites.main;
    // sprite.blit_to_buffer(frame, point2(
    //     (WIDTH / 2 - sprite.width / 2) as i32,
    //     (HEIGHT / 2 - sprite.height / 2) as i32,
    // ));

    // let mut dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);
    // let color = raqote::Color::new(0xff, 0xff, 0xff, 0xff);
    // let w = 50.;
    // let h = 70.;
    // let x = (WIDTH as f32 / 2.) - (w / 2.);
    // let y = (HEIGHT as f32 / 2.) - (h / 2.);
    // dt.fill_rect(x, y, w, h, &Source::Solid(SolidSource::from(color)), &DrawOptions::new());

    // let data = dt.get_data_u8();
    // frame.copy_from_slice(data);

    // for pixel in frame.chunks_exact_mut(4).skip((WIDTH * 100 + 100) as usize).take(10) {
    //     pixel[0] = 0xff;
    //     pixel[1] = 0xff;
    //     pixel[2] = 0xff;
    //     pixel[3] = 0xff;
    // }
}

fn exit_on_escape(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
