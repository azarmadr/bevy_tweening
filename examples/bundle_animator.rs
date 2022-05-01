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
        .add_system(bundle_animator::<Sprite,Transform>)
        .run();

    Ok(())
}

pub type Lerp<T> = dyn Fn(&mut T, &T, f32) + Send + Sync + 'static;
pub struct BeTween<T> {
    lerp: Box<Lerp<T>>,
    start: Option<T>,
}
impl<T> BeTween<T> {
    /// Construct a lens from lerping functions
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
impl<T: Clone> Lens<T> for BeTween<T> {
    fn lerp(&mut self, target: &mut T, ratio: f32) {
        if self.start.is_none() {
            self.start = Some(target.clone());
        }
        if let Some(start) = &self.start {
            (self.lerp)(target, start, ratio);
        }
    }
}

/// trying to create a bundled animator
pub fn bundle_animator<T:Component+Clone,R:Component+Clone>(
    time: Res<Time>,
    mut query: Query<(Entity, (&mut T,&mut R), &mut Animator<(T,R)>)>,
    mut event_writer: EventWriter<TweenCompleted>,
)
{
    for (entity, ref mut target, ref mut animator) in query.iter_mut() {
        if animator.state != AnimatorState::Paused {
            if let Some(tweenable) = animator.tweenable_mut() {
                let nt = &mut(target.0.clone(),target.1.clone());
                tweenable.tick(time.delta(), nt, entity, &mut event_writer);
                let (ref a,ref b) = nt;
                *target.0 = a.clone();
                *target.1 = b.clone();
            }
        }
    }
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
            BeTween::with_lerp(move |c: &mut(Sprite, Transform), _, r| {
                c.0.color = Vec4::from(Color::RED).lerp(Vec4::from(Color::BLUE),r).into();
                c.1.translation = Vec3::new(x, screen_y, 50.).lerp(Vec3::new(x, -screen_y, 50.), r);
                // rotation around y axis from 0 to 4pi(two rotations)
                c.1.rotation = Quat::from_rotation_y(0. + 4. * std::f32::consts::PI * r);
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