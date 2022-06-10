use crate::data::math::generator::gradient::GradientSerde;
use crate::data::math::interpolation::vector::VectorInterpolatorSerde;
use anyhow::Result;
use omg::data::math::generator::generator1d::Generator1d;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Generator1dSerde {
    AbsoluteGradient(GradientSerde),
    Gradient(GradientSerde),
    InputAsOutput,
    InterpolateVector(VectorInterpolatorSerde<u32, u8>),
}

type S = Generator1dSerde;
type R = Generator1d;

impl Generator1dSerde {
    pub fn try_convert(self) -> Result<Generator1d> {
        match self {
            S::AbsoluteGradient(gradient) => Ok(R::AbsoluteGradient(gradient.try_convert()?)),
            S::Gradient(gradient) => Ok(R::Gradient(gradient.try_convert()?)),
            S::InputAsOutput => Ok(R::InputAsOutput),
            S::InterpolateVector(interpolator) => {
                Ok(R::InterpolateVector(interpolator.try_convert()?))
            }
        }
    }
}

impl From<&Generator1d> for Generator1dSerde {
    fn from(generator: &Generator1d) -> Self {
        match generator {
            R::AbsoluteGradient(gradient) => S::AbsoluteGradient(gradient.into()),
            R::Gradient(gradient) => S::Gradient(gradient.into()),
            R::InputAsOutput => S::InputAsOutput,
            R::InterpolateVector(interpolator) => S::InterpolateVector(interpolator.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omg::data::math::generator::gradient::Gradient;

    #[test]
    fn test_convert_gradient() {
        let gradient = Gradient::new(1000, 500, 0, 255).unwrap();
        let start = R::AbsoluteGradient(gradient);
        let serde: Generator1dSerde = (&start).into();

        assert_eq!(serde.try_convert().unwrap(), start)
    }
}
