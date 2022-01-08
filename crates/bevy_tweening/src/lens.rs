use bevy::prelude::*;

/// A lens over a subset of a component.
///
/// The lens takes a `target` component or asset from a query, as a mutable reference,
/// and animates (tweens) a subet of the fields of the component/asset based on the
/// linear ratio `ratio`, already sampled from the easing curve.
///
/// # Example
///
/// Implement `Lens` for a custom type:
///
/// ```rust
/// struct MyLens {
///   start: f32,
///   end: f32,
/// }
///
/// struct MyStruct(f32);
///
/// impl Lens<MyStruct> for MyLens {
///   fn lerp(&mut self, target: &mut MyStruct, ratio: f32) {
///     target.0 = self.start + (self.end - self.start) * ratio;
///   }
/// }
/// ```
///
pub trait Lens<T> {
    /// Perform a linear interpolation (lerp) over the subset of fields of a component
    /// or asset the lens focuses on, based on the linear ratio `ratio`. The `target`
    /// component or asset is mutated in place. The implementation decides which fields
    /// are interpolated, and performs the animation in-place, overwriting the target.
    fn lerp(&mut self, target: &mut T, ratio: f32);
}

/// A lens to manipulate the [`color`] field of a section of a [`Text`] component.
///
/// [`color`]: bevy::text::TextStyle::color
/// [`Text`]: bevy::text::Text
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
    /// Index of the text section in the [`Text`] component.
    pub section: usize,
}

impl Lens<Text> for TextColorLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        let value = self.start + (self.end + self.start * -1.0) * ratio;
        target.sections[self.section].style.color = value;
    }
}

/// A lens to manipulate the [`translation`] field of a [`Transform`] component.
///
/// [`translation`]: bevy::transform::components::Transform::translation
/// [`Transform`]: bevy::transform::components::Transform
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformPositionLens {
    /// Start value of the translation.
    pub start: Vec3,
    /// End value of the translation.
    pub end: Vec3,
}

impl Lens<Transform> for TransformPositionLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.translation = value;
    }
}

/// A lens to manipulate the [`rotation`] field of a [`Transform`] component.
///
/// [`rotation`]: bevy::transform::components::Transform::rotation
/// [`Transform`]: bevy::transform::components::Transform
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformRotationLens {
    /// Start value of the rotation.
    pub start: Quat,
    /// End value of the rotation.
    pub end: Quat,
}

impl Lens<Transform> for TransformRotationLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.rotation = value;
    }
}

/// A lens to manipulate the [`scale`] field of a [`Transform`] component.
///
/// [`scale`]: bevy::transform::components::Transform::scale
/// [`Transform`]: bevy::transform::components::Transform
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformScaleLens {
    /// Start value of the scale.
    pub start: Vec3,
    /// End value of the scale.
    pub end: Vec3,
}

impl Lens<Transform> for TransformScaleLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.scale = value;
    }
}

/// A lens to manipulate the [`position`] field of a UI [`Style`] component.
///
/// [`position`]: bevy::ui::Style::position
/// [`Style`]: bevy::ui::Style
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiPositionLens {
    /// Start position.
    pub start: Rect<Val>,
    /// End position.
    pub end: Rect<Val>,
}

fn lerp_val(start: &Val, end: &Val, ratio: f32) -> Val {
    match (start, end) {
        (Val::Percent(start), Val::Percent(end)) => Val::Percent(start + (end - start) * ratio),
        (Val::Px(start), Val::Px(end)) => Val::Px(start + (end - start) * ratio),
        _ => *start,
    }
}

impl Lens<Style> for UiPositionLens {
    fn lerp(&mut self, target: &mut Style, ratio: f32) {
        target.position = Rect {
            left: lerp_val(&self.start.left, &self.end.left, ratio),
            right: lerp_val(&self.start.right, &self.end.right, ratio),
            top: lerp_val(&self.start.top, &self.end.top, ratio),
            bottom: lerp_val(&self.start.bottom, &self.end.bottom, ratio),
        }
    }
}

/// A lens to manipulate the [`color`] field of a [`ColorMaterial`] asset.
///
/// [`color`]: bevy::sprite::ColorMaterial::color
/// [`ColorMaterial`]: bevy::sprite::ColorMaterial
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorMaterialColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

impl Lens<ColorMaterial> for ColorMaterialColorLens {
    fn lerp(&mut self, target: &mut ColorMaterial, ratio: f32) {
        let value = self.start + (self.end + self.start * -1.) * ratio;
        target.color = value;
    }
}

/// A lens to manipulate the [`color`] field of a [`Sprite`] asset.
///
/// [`color`]: bevy::sprite::Sprite::color
/// [`Sprite`]: bevy::sprite::Sprite
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpriteColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

impl Lens<Sprite> for SpriteColorLens {
    fn lerp(&mut self, target: &mut Sprite, ratio: f32) {
        let value = self.start + (self.end + self.start * -1.) * ratio;
        target.color = value;
    }
}
