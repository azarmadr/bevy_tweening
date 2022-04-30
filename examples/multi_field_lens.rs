use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .insert_resource(WindowDescriptor {
            title: "TransformPositionLens".to_string(),
            width: 1400.,
            height: 600.,
            present_mode: bevy::window::PresentMode::Fifo, // vsync
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_startup_system(setup)
        .add_system(component_animator_system::<Visibility>)
        .run();

    Ok(())
}

pub type Lerp<T> = dyn Fn(&mut T, &T, f32) + Send + Sync + 'static;
pub struct BeTween<T> {
    lerp: Box<Lerp<T>>,
    start: Option<T>,
}
impl<T> BeTween<T> {
    /// Construct a lens from a pair of getter functions
    pub fn with_lerp<U>(lerp: U) -> Self
    where
        U: Fn(&mut T, &T, f32) + Send + Sync + 'static,
    {
        Self {
            lerp: Box::new(lerp),
            start: None,
        }
    }
}
impl<T: Clone> Lens<T> for BeTween<T>
where
    T: Component,
{
    fn lerp(&mut self, target: &mut T, ratio: f32) {
        if self.start.is_none() {
            self.start = Some(target.clone());
        }
        if let Some(start) = &self.start {
            (self.lerp)(target, start, ratio);
        }
    }
}

pub fn rot_seq(duration: std::time::Duration) -> Sequence<Transform> {
    let start = 0.;
    let end = std::f32::consts::PI / 2.;
    let tween = |start, end| {
        Tween::new(
            EaseFunction::QuadraticIn,
            TweeningType::Once,
            duration,
            TransformRotateYLens { start, end },
        )
    };
    tween(start, end).then(tween(end, start))
}
pub fn vis_seq(duration: std::time::Duration, show: bool) -> Tween<Visibility> {
    Tween::new(
        EaseFunction::QuadraticIn,
        TweeningType::Once,
        2 * duration,
        BeTween::with_lerp(move |c: &mut Visibility, _, r| c.is_visible = show ^ (r < 0.5)),
    )
}
pub fn shake_seq(duration: std::time::Duration) -> Sequence<Transform> {
    let tween = |s, e, i| {
        Tween::new(
            EaseFunction::ElasticInOut,
            TweeningType::Once,
            duration * i / 3,
            BeTween::with_lerp(move |c: &mut Transform, _, r| {
                c.rotation = Quat::from_rotation_z(s + (e - s) * r)
            }),
        )
    };
    let pi = std::f32::consts::PI;
    Sequence::new((1..4).rev().map(|i| {
        tween(0., pi / 12. / i as f32, i)
            .then(tween(-pi / 12. / i as f32, -pi / 12. / i as f32, i))
            .then(tween(-pi / 12. / i as f32, 0., i))
    }))
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let size = 25.;

    let spacing = 1.5;
    let screen_x = 570.;
    let screen_y = 150.;
    let mut x = -screen_x;

    for ease_function in &[
        EaseFunction::QuadraticIn,
        EaseFunction::QuadraticOut,
        EaseFunction::QuadraticInOut,
        EaseFunction::CubicIn,
        EaseFunction::CubicOut,
        EaseFunction::CubicInOut,
        EaseFunction::QuarticIn,
        EaseFunction::QuarticOut,
        EaseFunction::QuarticInOut,
        EaseFunction::QuinticIn,
        EaseFunction::QuinticOut,
        EaseFunction::QuinticInOut,
        EaseFunction::SineIn,
        EaseFunction::SineOut,
        EaseFunction::SineInOut,
        EaseFunction::CircularIn,
        EaseFunction::CircularOut,
        EaseFunction::CircularInOut,
        EaseFunction::ExponentialIn,
        EaseFunction::ExponentialOut,
        EaseFunction::ExponentialInOut,
        EaseFunction::ElasticIn,
        EaseFunction::ElasticOut,
        EaseFunction::ElasticInOut,
        EaseFunction::BackIn,
        EaseFunction::BackOut,
        EaseFunction::BackInOut,
        EaseFunction::BounceIn,
        EaseFunction::BounceOut,
        EaseFunction::BounceInOut,
    ] {
        let tween = Tween::new(
            *ease_function,
            TweeningType::PingPong,
            std::time::Duration::from_secs(3),
            //BeTween::with_lerp(move |c: &mut(Visibility, Transform), _, r| {
            BeTween::with_lerp(move |c: &mut Transform, _, r| {
                //c.0.is_visible = true ^ (r < 0.2);
                c.translation = Vec3::new(x, screen_y, 50.).lerp(Vec3::new(x, -screen_y, 50.),r);
                // rotation from s+delta*r
                c.rotation = Quat::from_rotation_y(0. + 4.*std::f32::consts::PI * r);
            }),
        );

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(size, size)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Animator::new(tween));

        x += size * spacing;
    }
}