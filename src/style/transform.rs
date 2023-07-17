use crate::{Scalar, Vector2};

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
    pub translate: Option<Vector2>,
    pub rotate: Option<Scalar>,
}

impl Into<Value> for Transform {
    fn into(self) -> Value {
        let transform_string = match (self.transform.translate, self.transform.rotate) {
            (Some(translation), Some(angle)) => format!(
                "translate({},{}) rotate({})",
                translation[0], translation[1], angle
            ),
            (Some(translation), None) => {
                format!("translate({},{})", translation[0], translation[1])
            }
            (None, Some(angle)) => format!("rotate({})", angle),
            (None, None) => "none",
        };

        transform_string.into()
    }
}
