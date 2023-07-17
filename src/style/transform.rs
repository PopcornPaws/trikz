use crate::svg::Value;
use crate::{Scalar, Vector2};

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
    pub translation: Option<Vector2>,
    pub rotation: Option<Scalar>,
}

impl From<Transform> for Value {
    fn from(transform: Transform) -> Value {
        let transform_string = match (transform.translation, transform.rotation) {
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

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn into_value() {
        assert_eq!(
            Value::from(Transform {
                translation: Some(Vector2::new(1.0, 2.0)),
                rotation: Some(90.0)
            })
            .deref(),
            Value::from("translate(1,2) rotate(90)").deref()
        );
        assert_eq!(
            Value::from(Transform {
                translation: Some(Vector2::new(1.0, 2.0)),
                rotation: None
            })
            .deref(),
            Value::from("translate(1,2)").deref()
        );
        assert_eq!(
            Value::from(Transform {
                translation: None,
                rotation: Some(90.0)
            })
            .deref(),
            Value::from("rotate(90)").deref()
        );
        assert_eq!(
            Value::from(Transform {
                translation: None,
                rotation: None
            })
            .deref(),
            Value::from("none").deref()
        );
    }
}
