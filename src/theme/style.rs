use super::{ColorStyle, Effect};
use enumset::EnumSet;

/// Combine a color and an effect.
///
/// Represents any transformation that can be applied to text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Style {
    /// Effect to apply.
    ///
    /// `None` to keep using previous effects.
    pub effects: EnumSet<Effect>,

    /// Color style to apply.
    ///
    /// `None` to keep using the previous colors.
    pub color: Option<ColorStyle>,
}

impl Style {
    /// Returns a new `Style` that doesn't apply anything.
    pub fn none() -> Self {
        Style {
            effects: EnumSet::new(),
            color: None,
        }
    }

    /// Returns a new `Style` by merging all given styles.
    ///
    /// Will use the last non-`None` color, and will combine all effects.
    pub fn merge(styles: &[Style]) -> Self {
        let mut color = None;
        let mut effects = EnumSet::new();

        for style in styles {
            if style.color.is_some() {
                color = style.color;
            }

            effects.insert_all(style.effects);
        }

        Style { color, effects }
    }
}

impl From<Effect> for Style {
    fn from(effect: Effect) -> Self {
        Style {
            effects: enum_set!(Effect, effect),
            color: None,
        }
    }
}

impl From<ColorStyle> for Style {
    fn from(color: ColorStyle) -> Self {
        Style {
            effects: EnumSet::new(),
            color: Some(color),
        }
    }
}
