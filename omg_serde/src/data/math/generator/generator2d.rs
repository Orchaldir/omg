use crate::data::math::generator::generator1d::Generator1dSerde;
use crate::data::math::size2d::Size2dSerde;
use anyhow::{Context, Result};
use omg::data::math::generator::generator1d::Generator1d;
use omg::data::math::generator::generator2d::Generator2d;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Generator2dSerde {
    ApplyToX(Generator1dSerde),
    ApplyToY(Generator1dSerde),
    ApplyToDistance {
        generator: Generator1dSerde,
        center_x: u32,
        center_y: u32,
    },
    IndexGenerator(Size2dSerde),
}

type S = Generator2dSerde;
type R = Generator2d;

impl Generator2dSerde {
    pub fn try_convert(self) -> Result<Generator2d> {
        match self {
            S::ApplyToX(data) => {
                let generator = data
                    .try_convert()
                    .context("Failed to convert to Generator2d::ApplyToX!")?;
                Ok(R::ApplyToX(generator))
            }
            S::ApplyToY(data) => {
                let generator = data
                    .try_convert()
                    .context("Failed to convert to Generator2d::ApplyToY!")?;
                Ok(R::ApplyToY(generator))
            }
            S::ApplyToDistance {
                generator,
                center_x,
                center_y,
            } => {
                let generator: Generator1d = generator
                    .try_convert()
                    .context("Failed to convert to Generator2d::ApplyToDistance!")?;
                Ok(Generator2d::new_apply_to_distance(
                    generator, center_x, center_y,
                ))
            }
            S::IndexGenerator(size) => {
                let size = size
                    .try_convert()
                    .context("Failed to convert to Generator2d::IndexGenerator!")?;
                Ok(R::IndexGenerator(size))
            }
        }
    }
}

impl From<&Generator2d> for Generator2dSerde {
    fn from(generator: &Generator2d) -> Self {
        match generator {
            R::ApplyToX(generator) => S::ApplyToX(generator.into()),
            R::ApplyToY(generator) => S::ApplyToY(generator.into()),
            R::ApplyToDistance {
                generator,
                center_x,
                center_y,
            } => S::ApplyToDistance {
                generator: generator.into(),
                center_x: *center_x,
                center_y: *center_y,
            },
            R::IndexGenerator(size) => S::IndexGenerator(size.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg::data::math::size2d::Size2d;

    #[test]
    fn test_convert_apply_to_x() {
        assert_eq(Generator2d::ApplyToX(Generator1d::InputAsOutput))
    }

    #[test]
    fn test_convert_apply_to_y() {
        assert_eq(Generator2d::ApplyToY(Generator1d::InputAsOutput))
    }

    #[test]
    fn test_convert_apply_to_distance() {
        assert_eq(Generator2d::ApplyToDistance {
            generator: Generator1d::InputAsOutput,
            center_x: 1,
            center_y: 2,
        })
    }

    #[test]
    fn test_convert_index_generator() {
        assert_eq(Generator2d::IndexGenerator(Size2d::unchecked(1, 2)))
    }

    fn assert_eq(start: Generator2d) {
        let serde: Generator2dSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}
