//! Collection of predefined lenses for common Bevy components and assets.
//!
//! # Predefined lenses
//!
//! This module contains predefined lenses for common use cases. Those lenses
//! are entirely optional. They can be used if they fit your use case, to save
//! some time, but are not treated any differently from a custom user-provided
//! lens.
//!
//! # Rotations
//!
//! Several rotation lenses are provided, with different properties.
//!
//! ## Shortest-path rotation
//!
//! The [`TransformRotationLens`] animates the [`rotation`] field of a
//! [`Transform`] component using [`Quat::slerp()`]. It inherits the properties
//! of that method, and in particular the fact it always finds the "shortest
//! path" from start to end. This is well suited for animating a rotation
//! between two given directions, but will provide unexpected results if you try
//! to make an entity rotate around a given axis for more than half a turn, as
//! [`Quat::slerp()`] will then try to move "the other way around".
//!
//! ## Angle-focused rotations
//!
//! Conversely, for cases where the rotation direction is important, like when
//! trying to do a full 360-degree turn, a series of angle-based interpolation
//! lenses is provided:
//! - [`TransformRotateXLens`]
//! - [`TransformRotateYLens`]
//! - [`TransformRotateZLens`]
//! - [`TransformRotateAxisLens`]
//!
//! [`rotation`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html#structfield.rotation
//! [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
//! [`Quat::slerp()`]: https://docs.rs/bevy/0.16.0/bevy/math/struct.Quat.html#method.slerp

use bevy::prelude::*;

use crate::Targetable;

use crate::TweeningDirection;

/// A lens over a subset of a component.
///
/// The lens takes a `target` component or asset from a query, as a mutable
/// reference, and animates (tweens) a subset of the fields of the
/// component/asset based on the linear ratio `ratio` in \[0:1\], already
/// sampled from the easing curve.
///
/// # Example
///
/// Implement `Lens` for a custom type:
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_tweening::*;
/// struct MyLens {
///   start: f32,
///   end: f32,
/// }
///
/// #[derive(Component)]
/// struct MyStruct(f32);
///
/// impl Lens<MyStruct> for MyLens {
///   fn lerp(&mut self, target: &mut dyn Targetable<MyStruct>, ratio: f32) {
///     target.0 = self.start + (self.end - self.start) * ratio;
///   }
/// }
/// ```
pub trait Lens<T> {
    /// Perform a linear interpolation (lerp) over the subset of fields of a
    /// component or asset the lens focuses on, based on the linear ratio
    /// `ratio`. The `target` component or asset is mutated in place. The
    /// implementation decides which fields are interpolated, and performs
    /// the animation in-place, overwriting the target.
    fn lerp(&mut self, target: &mut dyn Targetable<T>, ratio: f32);

    /// Update lens on tween start
    /// Can be used for relative lenses
    #[allow(unused_variables)]
    fn update_on_tween_start(
        &mut self,
        target: &mut dyn Targetable<T>,
        direction: TweeningDirection,
        times_completed: i32,
    ) {
    }
}

/// A lens to manipulate the [`color`] field of a section of a [`Text`]
/// component.
///
/// [`color`]: https://docs.rs/bevy/0.16.0/bevy/text/struct.TextColor.html
#[cfg(feature = "bevy_text")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

#[cfg(feature = "bevy_text")]
impl Lens<TextColor> for TextColorLens {
    fn lerp(&mut self, target: &mut dyn Targetable<TextColor>, ratio: f32) {
        target.0 = self.start.mix(&self.end, ratio);
    }
}

/// A lens to manipulate the [`translation`] field of a [`Transform`] component.
///
/// [`translation`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html#structfield.translation
/// [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformPositionLens {
    /// Start value of the translation.
    pub start: Vec3,
    /// End value of the translation.
    pub end: Vec3,
}

impl Lens<Transform> for TransformPositionLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.translation = value;
    }
}

/// A lens to manipulate the [`rotation`] field of a [`Transform`] component.
///
/// This lens interpolates the [`rotation`] field of a [`Transform`] component
/// from a `start` value to an `end` value using the spherical linear
/// interpolation provided by [`Quat::slerp()`]. This means the rotation always
/// uses the shortest path from `start` to `end`. In particular, this means it
/// cannot make entities do a full 360 degrees turn. Instead use
/// [`TransformRotateXLens`] and similar to interpolate the rotation angle
/// around a given axis.
///
/// See the [top-level `lens` module documentation] for a comparison of rotation
/// lenses.
///
/// [`rotation`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html#structfield.rotation
/// [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
/// [`Quat::slerp()`]: https://docs.rs/bevy/0.16.0/bevy/math/struct.Quat.html#method.slerp
/// [top-level `lens` module documentation]: crate::lens
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformRotationLens {
    /// Start value of the rotation.
    pub start: Quat,
    /// End value of the rotation.
    pub end: Quat,
}

impl Lens<Transform> for TransformRotationLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        target.rotation = self.start.slerp(self.end, ratio);
    }
}

/// A lens to rotate a [`Transform`] component around its local X axis.
///
/// This lens interpolates the rotation angle of a [`Transform`] component from
/// a `start` value to an `end` value, for a rotation around the X axis. Unlike
/// [`TransformRotationLens`], it can produce an animation that rotates the
/// entity any number of turns around its local X axis.
///
/// See the [top-level `lens` module documentation] for a comparison of rotation
/// lenses.
///
/// [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
/// [top-level `lens` module documentation]: crate::lens
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformRotateXLens {
    /// Start value of the rotation angle, in radians.
    pub start: f32,
    /// End value of the rotation angle, in radians.
    pub end: f32,
}

impl Lens<Transform> for TransformRotateXLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let angle = (self.end - self.start).mul_add(ratio, self.start);
        target.rotation = Quat::from_rotation_x(angle);
    }
}

/// A lens to rotate a [`Transform`] component around its local Y axis.
///
/// This lens interpolates the rotation angle of a [`Transform`] component from
/// a `start` value to an `end` value, for a rotation around the Y axis. Unlike
/// [`TransformRotationLens`], it can produce an animation that rotates the
/// entity any number of turns around its local Y axis.
///
/// See the [top-level `lens` module documentation] for a comparison of rotation
/// lenses.
///
/// [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
/// [top-level `lens` module documentation]: crate::lens
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformRotateYLens {
    /// Start value of the rotation angle, in radians.
    pub start: f32,
    /// End value of the rotation angle, in radians.
    pub end: f32,
}

impl Lens<Transform> for TransformRotateYLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let angle = (self.end - self.start).mul_add(ratio, self.start);
        target.rotation = Quat::from_rotation_y(angle);
    }
}

/// A lens to rotate a [`Transform`] component around its local Z axis.
///
/// This lens interpolates the rotation angle of a [`Transform`] component from
/// a `start` value to an `end` value, for a rotation around the Z axis. Unlike
/// [`TransformRotationLens`], it can produce an animation that rotates the
/// entity any number of turns around its local Z axis.
///
/// See the [top-level `lens` module documentation] for a comparison of rotation
/// lenses.
///
/// [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
/// [top-level `lens` module documentation]: crate::lens
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformRotateZLens {
    /// Start value of the rotation angle, in radians.
    pub start: f32,
    /// End value of the rotation angle, in radians.
    pub end: f32,
}

impl Lens<Transform> for TransformRotateZLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let angle = (self.end - self.start).mul_add(ratio, self.start);
        target.rotation = Quat::from_rotation_z(angle);
    }
}

/// A lens to rotate a [`Transform`] component around a given fixed axis.
///
/// This lens interpolates the rotation angle of a [`Transform`] component from
/// a `start` value to an `end` value, for a rotation around a given axis.
/// Unlike [`TransformRotationLens`], it can produce an animation that rotates
/// the entity any number of turns around that axis.
///
/// See the [top-level `lens` module documentation] for a comparison of rotation
/// lenses.
///
/// # Panics
///
/// This method panics if the `axis` vector is not normalized.
///
/// [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
/// [top-level `lens` module documentation]: crate::lens
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformRotateAxisLens {
    /// The normalized rotation axis.
    pub axis: Vec3,
    /// Start value of the rotation angle, in radians.
    pub start: f32,
    /// End value of the rotation angle, in radians.
    pub end: f32,
}

impl Lens<Transform> for TransformRotateAxisLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        let angle = (self.end - self.start).mul_add(ratio, self.start);
        target.rotation = Quat::from_axis_angle(self.axis, angle);
    }
}

/// A lens to manipulate the [`scale`] field of a [`Transform`] component.
///
/// [`scale`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html#structfield.scale
/// [`Transform`]: https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformScaleLens {
    /// Start value of the scale.
    pub start: Vec3,
    /// End value of the scale.
    pub end: Vec3,
}

impl Lens<Transform> for TransformScaleLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
        target.scale = self.start + (self.end - self.start) * ratio;
    }
}

/// A lens to manipulate the [`position`] field of a UI [`Node`] component.
///
/// [`position`]: https://docs.rs/bevy/0.16.0/bevy/ui/struct.Node.html
/// [`Node`]: https://docs.rs/bevy/0.16.0/bevy/ui/struct.Node.html
#[cfg(feature = "bevy_ui")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiPositionLens {
    /// Start position.
    pub start: UiRect,
    /// End position.
    pub end: UiRect,
}

#[cfg(feature = "bevy_ui")]
fn lerp_val(start: &Val, end: &Val, ratio: f32) -> Val {
    match (start, end) {
        (Val::Percent(start), Val::Percent(end)) => {
            Val::Percent((end - start).mul_add(ratio, *start))
        }
        (Val::Px(start), Val::Px(end)) => Val::Px((end - start).mul_add(ratio, *start)),
        (Val::Vw(start), Val::Vw(end)) => Val::Vw((end - start).mul_add(ratio, *start)),
        (Val::Vh(start), Val::Vh(end)) => Val::Vh((end - start).mul_add(ratio, *start)),
        (Val::VMin(start), Val::VMin(end)) => Val::VMin((end - start).mul_add(ratio, *start)),
        (Val::VMax(start), Val::VMax(end)) => Val::VMax((end - start).mul_add(ratio, *start)),
        _ => *start,
    }
}

#[cfg(feature = "bevy_ui")]
impl Lens<Node> for UiPositionLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Node>, ratio: f32) {
        target.left = lerp_val(&self.start.left, &self.end.left, ratio);
        target.right = lerp_val(&self.start.right, &self.end.right, ratio);
        target.top = lerp_val(&self.start.top, &self.end.top, ratio);
        target.bottom = lerp_val(&self.start.bottom, &self.end.bottom, ratio);
    }
}

/// Gamer
#[cfg(feature = "bevy_ui")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiBackgroundColorLens {
    /// Start position.
    pub start: Color,
    /// End position.
    pub end: Color,
}

#[cfg(feature = "bevy_ui")]
impl Lens<BackgroundColor> for UiBackgroundColorLens {
    fn lerp(&mut self, target: &mut dyn Targetable<BackgroundColor>, ratio: f32) {
        target.0 = self.start.mix(&self.end, ratio);
    }
}

/// A lens to manipulate the [`color`] field of a [`ColorMaterial`] asset.
///
/// [`color`]: https://docs.rs/bevy/0.16.0/bevy/sprite/struct.ColorMaterial.html#structfield.color
/// [`ColorMaterial`]: https://docs.rs/bevy/0.16.0/bevy/sprite/struct.ColorMaterial.html
#[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorMaterialColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

#[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
impl Lens<ColorMaterial> for ColorMaterialColorLens {
    fn lerp(&mut self, target: &mut dyn Targetable<ColorMaterial>, ratio: f32) {
        target.color = self.start.mix(&self.end, ratio);
    }
}

/// A lens to manipulate the [`color`] field of a [`Sprite`] asset.
///
/// [`color`]: https://docs.rs/bevy/0.16.0/bevy/sprite/struct.Sprite.html#structfield.color
/// [`Sprite`]: https://docs.rs/bevy/0.16.0/bevy/sprite/struct.Sprite.html
#[cfg(feature = "bevy_sprite")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpriteColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

#[cfg(feature = "bevy_sprite")]
impl Lens<Sprite> for SpriteColorLens {
    fn lerp(&mut self, target: &mut dyn Targetable<Sprite>, ratio: f32) {
        let value = self.start.mix(&self.end, ratio);
        target.color = value;
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::{change_detection::MaybeLocation, component::Tick};
    use std::f32::consts::TAU;

    #[cfg(any(feature = "bevy_sprite", feature = "bevy_text"))]
    use bevy::color::palettes::css::{BLUE, RED};

    use super::*;

    use crate::tweenable::ComponentTarget;

    #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
    use crate::tweenable::AssetTarget;

    #[cfg(feature = "bevy_text")]
    #[test]
    fn text_color() {
        let mut lens = TextColorLens {
            start: RED.into(),
            end: BLUE.into(),
        };

        let mut text_color = TextColor::default();

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut text_color,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.);
        }
        assert_eq!(text_color.0, RED.into());

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut text_color,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 1.);
        }
        assert_eq!(text_color.0, BLUE.into());

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut text_color,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.3);
        }
        assert_eq!(text_color.0, Color::srgba(0.7, 0., 0.3, 1.0));
    }

    #[test]
    fn transform_position() {
        let mut lens = TransformPositionLens {
            start: Vec3::ZERO,
            end: Vec3::new(1., 2., -4.),
        };
        let mut transform = Transform::default();

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 1.);
        }
        assert!(transform
            .translation
            .abs_diff_eq(Vec3::new(1., 2., -4.), 1e-5));
        assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.3);
        }
        assert!(transform
            .translation
            .abs_diff_eq(Vec3::new(0.3, 0.6, -1.2), 1e-5));
        assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
    }

    #[test]
    fn transform_rotation() {
        let mut lens = TransformRotationLens {
            start: Quat::IDENTITY,
            end: Quat::from_rotation_z(100_f32.to_radians()),
        };
        let mut transform = Transform::default();

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 1.);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(100_f32.to_radians()), 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.3);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(30_f32.to_radians()), 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
    }

    #[test]
    fn transform_rotate_x() {
        let mut lens = TransformRotateXLens {
            start: 0.,
            end: 1440_f32.to_radians(), // 4 turns
        };
        let mut transform = Transform::default();

        for (index, ratio) in [0., 0.25, 0.5, 0.75, 1.].iter().enumerate() {
            {
                let mut added = Tick::new(0);
                let mut last_changed = Tick::new(0);
                let mut caller = MaybeLocation::caller();
                let mut target = ComponentTarget::new(Mut::new(
                    &mut transform,
                    &mut added,
                    &mut last_changed,
                    Tick::new(0),
                    Tick::new(0),
                    caller.as_mut(),
                ));

                lens.lerp(&mut target, *ratio);
            }
            assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
            if index == 1 || index == 3 {
                // For odd-numbered turns, the opposite Quat is produced. This is equivalent in
                // terms of rotation to the IDENTITY one, but numerically the w component is not
                // the same so would fail an equality test.
                assert!(transform
                    .rotation
                    .abs_diff_eq(Quat::from_xyzw(0., 0., 0., -1.), 1e-5));
            } else {
                assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
            }
            assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
        }

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.1);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform
            .rotation
            .abs_diff_eq(Quat::from_rotation_x(0.1 * (4. * TAU)), 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
    }

    #[test]
    fn transform_rotate_y() {
        let mut lens = TransformRotateYLens {
            start: 0.,
            end: 1440_f32.to_radians(), // 4 turns
        };
        let mut transform = Transform::default();

        for (index, ratio) in [0., 0.25, 0.5, 0.75, 1.].iter().enumerate() {
            {
                let mut added = Tick::new(0);
                let mut last_changed = Tick::new(0);
                let mut caller = MaybeLocation::caller();
                let mut target = ComponentTarget::new(Mut::new(
                    &mut transform,
                    &mut added,
                    &mut last_changed,
                    Tick::new(0),
                    Tick::new(0),
                    caller.as_mut(),
                ));

                lens.lerp(&mut target, *ratio);
            }
            assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
            if index == 1 || index == 3 {
                // For odd-numbered turns, the opposite Quat is produced. This is equivalent in
                // terms of rotation to the IDENTITY one, but numerically the w component is not
                // the same so would fail an equality test.
                assert!(transform
                    .rotation
                    .abs_diff_eq(Quat::from_xyzw(0., 0., 0., -1.), 1e-5));
            } else {
                assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
            }
            assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
        }

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.1);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform
            .rotation
            .abs_diff_eq(Quat::from_rotation_y(0.1 * (4. * TAU)), 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
    }

    #[test]
    fn transform_rotate_z() {
        let mut lens = TransformRotateZLens {
            start: 0.,
            end: 1440_f32.to_radians(), // 4 turns
        };
        let mut transform = Transform::default();

        for (index, ratio) in [0., 0.25, 0.5, 0.75, 1.].iter().enumerate() {
            {
                let mut added = Tick::new(0);
                let mut last_changed = Tick::new(0);
                let mut caller = MaybeLocation::caller();
                let mut target = ComponentTarget::new(Mut::new(
                    &mut transform,
                    &mut added,
                    &mut last_changed,
                    Tick::new(0),
                    Tick::new(0),
                    caller.as_mut(),
                ));

                lens.lerp(&mut target, *ratio);
            }
            assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
            if index == 1 || index == 3 {
                // For odd-numbered turns, the opposite Quat is produced. This is equivalent in
                // terms of rotation to the IDENTITY one, but numerically the w component is not
                // the same so would fail an equality test.
                assert!(transform
                    .rotation
                    .abs_diff_eq(Quat::from_xyzw(0., 0., 0., -1.), 1e-5));
            } else {
                assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
            }
            assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
        }

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.1);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(0.1 * (4. * TAU)), 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
    }

    #[test]
    fn transform_rotate_axis() {
        let axis = Vec3::ONE.normalize();
        let mut lens = TransformRotateAxisLens {
            axis,
            start: 0.,
            end: 1440_f32.to_radians(), // 4 turns
        };
        let mut transform = Transform::default();

        for (index, ratio) in [0., 0.25, 0.5, 0.75, 1.].iter().enumerate() {
            {
                let mut added = Tick::new(0);
                let mut last_changed = Tick::new(0);
                let mut caller = MaybeLocation::caller();
                let mut target = ComponentTarget::new(Mut::new(
                    &mut transform,
                    &mut added,
                    &mut last_changed,
                    Tick::new(0),
                    Tick::new(0),
                    caller.as_mut(),
                ));

                lens.lerp(&mut target, *ratio);
            }
            assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
            if index == 1 || index == 3 {
                // For odd-numbered turns, the opposite Quat is produced. This is equivalent in
                // terms of rotation to the IDENTITY one, but numerically the w component is not
                // the same so would fail an equality test.
                assert!(transform
                    .rotation
                    .abs_diff_eq(Quat::from_xyzw(0., 0., 0., -1.), 1e-5));
            } else {
                assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
            }
            assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
        }

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.1);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform
            .rotation
            .abs_diff_eq(Quat::from_axis_angle(axis, 0.1 * (4. * TAU)), 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ONE, 1e-5));
    }

    #[test]
    fn transform_scale() {
        let mut lens = TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::new(1., 2., -4.),
        };
        let mut transform = Transform::default();

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::ZERO, 1e-5));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 1.);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::new(1., 2., -4.), 1e-5));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut transform,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.3);
        }
        assert!(transform.translation.abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!(transform.rotation.abs_diff_eq(Quat::IDENTITY, 1e-5));
        assert!(transform.scale.abs_diff_eq(Vec3::new(0.3, 0.6, -1.2), 1e-5));
    }

    #[cfg(feature = "bevy_ui")]
    #[test]
    fn ui_position() {
        let mut lens = UiPositionLens {
            start: UiRect {
                left: Val::Px(0.),
                top: Val::Px(0.),
                right: Val::Auto,
                bottom: Val::Percent(25.),
            },
            end: UiRect {
                left: Val::Px(1.),
                top: Val::Px(5.),
                right: Val::Auto,
                bottom: Val::Percent(45.),
            },
        };
        let mut node = Node::default();

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut node,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.);
        }
        assert_eq!(node.left, Val::Px(0.));
        assert_eq!(node.top, Val::Px(0.));
        assert_eq!(node.right, Val::Auto);
        assert_eq!(node.bottom, Val::Percent(25.));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut node,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 1.);
        }
        assert_eq!(node.left, Val::Px(1.));
        assert_eq!(node.top, Val::Px(5.));
        assert_eq!(node.right, Val::Auto);
        assert_eq!(node.bottom, Val::Percent(45.));

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut node,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.3);
        }
        assert_eq!(node.left, Val::Px(0.3));
        assert_eq!(node.top, Val::Px(1.5));
        assert_eq!(node.right, Val::Auto);
        assert_eq!(node.bottom, Val::Percent(31.));
    }

    #[cfg(all(feature = "bevy_sprite", feature = "bevy_asset"))]
    #[test]
    fn colormaterial_color() {
        let mut lens = ColorMaterialColorLens {
            start: RED.into(),
            end: BLUE.into(),
        };
        let mut assets = Assets::default();
        let handle = assets.add(ColorMaterial {
            color: Color::WHITE,
            texture: None,
            ..default()
        });

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = AssetTarget::new(Mut::new(
                &mut assets,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));
            target.handle = handle.clone();

            lens.lerp(&mut target, 0.);
        }
        assert_eq!(assets.get(handle.id()).unwrap().color, RED.into());

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = AssetTarget::new(Mut::new(
                &mut assets,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));
            target.handle = handle.clone();

            lens.lerp(&mut target, 1.);
        }
        assert_eq!(assets.get(handle.id()).unwrap().color, BLUE.into());

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = AssetTarget::new(Mut::new(
                &mut assets,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));
            target.handle = handle.clone();

            lens.lerp(&mut target, 0.3);
        }
        assert_eq!(
            assets.get(handle.id()).unwrap().color,
            Color::srgba(0.7, 0., 0.3, 1.0)
        );
    }

    #[cfg(feature = "bevy_sprite")]
    #[test]
    fn sprite_color() {
        let mut lens = SpriteColorLens {
            start: RED.into(),
            end: BLUE.into(),
        };
        let mut sprite = Sprite {
            color: Color::WHITE,
            ..default()
        };

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut sprite,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.);
        }
        assert_eq!(sprite.color, RED.into());

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut sprite,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 1.);
        }
        assert_eq!(sprite.color, BLUE.into());

        {
            let mut added = Tick::new(0);
            let mut last_changed = Tick::new(0);
            let mut caller = MaybeLocation::caller();
            let mut target = ComponentTarget::new(Mut::new(
                &mut sprite,
                &mut added,
                &mut last_changed,
                Tick::new(0),
                Tick::new(0),
                caller.as_mut(),
            ));

            lens.lerp(&mut target, 0.3);
        }
        assert_eq!(sprite.color, Color::srgba(0.7, 0., 0.3, 1.0));
    }
}
