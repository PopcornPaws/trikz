use crate::svg::Value;
use crate::{Scalar, Vector2};

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
    pub translation: Option<Vector2>,
    pub rotation: Option<Scalar>,
}

impl Into<Value> for Transform {
    fn into(self) -> Value {
        let transform_string = match (self.translation, self.rotation) {
            (Some(translation), Some(angle)) => format!(
                "translate({},{}) rotate({})",
                translation[0], translation[1], angle
            ),
            (Some(translation), None) => {
                format!("translate({},{})", translation[0], translation[1])
            }
            (None, Some(angle)) => format!("rotate({})", angle),
            (None, None) => "none".to_string(),
        };

        transform_string.into()
    }
}
