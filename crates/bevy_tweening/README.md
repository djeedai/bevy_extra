# Bevy Tweening

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Doc](https://docs.rs/bevy_tweening/badge.svg)](https://docs.rs/bevy_tweening) [![Crate](https://img.shields.io/crates/v/bevy_tweening.svg)](https://crates.io/crates/bevy_tweening)

Tweening animation plugin for Bevy.

## Usage

### System setup

Add the tweening plugin to your app:

```rust
App::default()
    .add_default_plugins()
    .add_plugin(TweeningPlugin)
    .run();
```

### Animate a component

Animate the transform position of an entity:

```rust
commands
    // Spawn a Sprite entity to animate the position of
    .spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(size, size)),
            ..Default::default()
        },
        ..Default::default()
    })
    // Add an Animator component to perform the animation
    .insert(Animator::new(
        // Use a quadratic easing on both endpoints
        EaseFunction::QuadraticInOut,
        // Loop animation back and forth over 1 second, with a 0.5 second
        // pause after each cycle (start -> end -> start).
        TweeningType::PingPong {
            duration: Duration::from_secs(1),
            pause: Some(Duration::from_millis(500)),
        },
        // The lens gives access to the Transform component of the Sprite,
        // for the Animator to animate it. It also contains the start and
        // end values associated with the animation ratios 0. and 1.
        TransformPositionLens {
            start: Vec3::new(0., 0., 0.),
            end: Vec3::new(1., 2., -4.),
        },
    ));
```

## Predefined Lenses

The naming scheme for predefined lenses is `"<TargetName><FieldName>Lens"`, where `<TargetName>` is the name of the target component or asset which is queried, and `<FieldName>` is the field which is mutated in place.

### Bevy Components

| Target Component | Animated Field | Lens |
|---|---|---|
| [`Sprite`](https://docs.rs/bevy/0.6.0/bevy/sprite/struct.Sprite.html) | [`color`](https://docs.rs/bevy/0.6.0/bevy/sprite/struct.Sprite.html#structfield.color) | [`SpriteColorLens`](https://docs.rs/bevy_tweening/latest/bevy_tweening/struct.SpriteColorLens.html) |
| [`Transform`](https://docs.rs/bevy/0.6.0/bevy/transform/components/struct.Transform.html) | [`translation`](https://docs.rs/bevy/0.6.0/bevy/transform/components/struct.Transform.html#structfield.translation) | [`TransformPositionLens`](https://docs.rs/bevy_tweening/latest/bevy_tweening/struct.TransformPositionLens.html) |
| | [`rotation`](https://docs.rs/bevy/0.6.0/bevy/transform/components/struct.Transform.html#structfield.rotation) | [`TransformRotationLens`](https://docs.rs/bevy_tweening/latest/bevy_tweening/struct.TransformRotationLens.html) |
| | [`scale`](https://docs.rs/bevy/0.6.0/bevy/transform/components/struct.Transform.html#structfield.scale) | [`TransformScaleLens`](https://docs.rs/bevy_tweening/latest/bevy_tweening/struct.TransformScaleLens.html) |
| [`Style`](https://docs.rs/bevy/0.6.0/bevy/ui/struct.Style.html) | [`position`](https://docs.rs/bevy/0.6.0/bevy/ui/struct.Style.html#structfield.position) | [`UiPositionLens`](https://docs.rs/bevy_tweening/latest/bevy_tweening/struct.UiPositionLens.html) |
| [`Text`](https://docs.rs/bevy/0.6.0/bevy/text/struct.Text.html) | [`TextStyle::color`](https://docs.rs/bevy/0.6.0/bevy/text/struct.TextStyle.html#structfield.color) | [`TextColorLens`](https://docs.rs/bevy_tweening/latest/bevy_tweening/struct.TextColorLens.html) |

### Bevy Assets

| Target Asset | Animated Field | Lens |
|---|---|---|
| [`ColorMaterial`](https://docs.rs/bevy/0.6.0/bevy/sprite/struct.ColorMaterial.html) | [`color`](https://docs.rs/bevy/0.6.0/bevy/sprite/struct.ColorMaterial.html#structfield.color) | [`ColorMaterialColorLens`](https://docs.rs/bevy_tweening/latest/bevy_tweening/struct.ColorMaterialColorLens.html) |

### Custom component support

To be able to animate some fields of a custom component, a custom lens need to be implemented for that component, which **linearly** interpolates the field(s) of that component.

```rust
#[derive(Component)]
struct CustomComponent(f32);

struct CustomLens {
    start: f32,
    end: f32,
}

impl Lens<CustomComponent> for CustomLens {
    fn lerp(&self, target: &mut CustomComponent, ratio: f32) -> f32 {
        target.0 = self.start + (self.end - self.start) * ratio;
    }
}
```

This process can also be used to interpolate fields of existing Bevy built-in components for which a predfined lens is not provided.

The basic formula for lerp (linear interpolation) is either of:

- `start + (end - start) * scalar`
- `start * (1.0 - scalar) + end * scalar`

The two formulations are mathematically equivalent, but one may be more suited than the other depending on the type interpolated and the operations available, and the potential floating-point precision errors.

Then, the system `component_animator_system::<CustomComponent>` needs to be added to the application.

## Custom asset support

The process is similar to custom components, creating a custom lens for the custom asset. The system to add is `asset_animator_system::<CustomAsset>`.

## Examples

See the [`examples/`](https://github.com/djeedai/bevy_extra/tree/main/crates/bevy_tweening/examples) folder.

### [`sprite_color`](examples/sprite_color.rs)

```rust
cargo run --example sprite_color --features="bevy/bevy_winit"
```

![sprite_color](https://raw.githubusercontent.com/djeedai/bevy_extra/main/crates/bevy_tweening/examples/sprite_color.gif)

### [`transform_rotation`](examples/transform_rotation.rs)

```rust
cargo run --example transform_rotation --features="bevy/bevy_winit"
```

![sprite_color](https://raw.githubusercontent.com/djeedai/bevy_extra/main/crates/bevy_tweening/examples/transform_rotation.gif)

### [`transform_translation`](examples/transform_translation.rs)

```rust
cargo run --example transform_translation --features="bevy/bevy_winit"
```

![sprite_color](https://raw.githubusercontent.com/djeedai/bevy_extra/main/crates/bevy_tweening/examples/transform_translation.gif)

### [`colormaterial_color`](examples/colormaterial_color.rs)

```rust
cargo run --example colormaterial_color --features="bevy/bevy_winit"
```

![colormaterial_color](https://raw.githubusercontent.com/djeedai/bevy_extra/main/crates/bevy_tweening/examples/colormaterial_color.gif)

## Ease Functions

Many [ease functions](https://docs.rs/interpolation/0.2.0/interpolation/enum.EaseFunction.html) are available:

- QuadraticIn
- QuadraticOut
- QuadraticInOut
- CubicIn
- CubicOut
- CubicInOut
- QuarticIn
- QuarticOut
- QuarticInOut
- QuinticIn
- QuinticOut
- QuinticInOut
- SineIn
- SineOut
- SineInOut
- CircularIn
- CircularOut
- CircularInOut
- ExponentialIn
- ExponentialOut
- ExponentialInOut
- ElasticIn
- ElasticOut
- ElasticInOut
- BackIn
- BackOut
- BackInOut
- BounceIn
- BounceOut
- BounceInOut

## Comparison with `bevy_easings`

The `bevy_tweening` library started as a fork of [the `bevy_easings` library by François Mocker](https://github.com/mockersf/bevy_extra/blob/master/bevy_easings), with the goals to:

- explore an alternative design based on lenses instead of generic types for each easer/animator. This reduces both the number of generic types needed, and hopefully the code size, as well as the number of systems needed to perform the interpolation.
- improve the interpolation of assets to avoid creating many copies like `bevy_easings` does, and instead mutate the assets (and, by similarity, the components too) in-place without making a copy. The in-place mutation also allows a more optimal interpolation limited to modifying the fields of interest only, instead of creating a new copy of the entire component each tick.
