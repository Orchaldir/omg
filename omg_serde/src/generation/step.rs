use crate::generation::attributes::create::CreateAttributeStepSerde;
use crate::generation::attributes::distortion1d::Distortion1dStepSerde;
use crate::generation::attributes::distortion2d::Distortion2dStepSerde;
use crate::generation::attributes::generator::GeneratorStepSerde;
use crate::generation::attributes::modify::ModifyWithAttributeStepSerde;
use crate::generation::attributes::transformer::TransformAttribute2dStepSerde;
use anyhow::{Context, Result};
use omg::generation::step::GenerationStep;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub fn get_attribute_id(attribute: &str, attributes: &[String]) -> Result<usize> {
    attributes
        .iter()
        .position(|name| name.eq(attribute))
        .with_context(|| format!("Unknown attribute '{}'", attribute))
}

pub trait ToStep<T> {
    fn try_convert(self, attributes: &mut Vec<String>) -> Result<T>;
}

pub trait FromStep<T> {
    fn convert(&self, attributes: &mut Vec<String>) -> T;
}

pub fn assert_eq<R: FromStep<S> + PartialEq + Debug, S: ToStep<R>>(
    step: R,
    attributes: &mut Vec<String>,
) {
    let serde: S = (&step).convert(attributes);

    assert_eq!(serde.try_convert(attributes).unwrap(), step)
}

#[derive(new, Debug, Serialize, Deserialize)]
pub enum GenerationStepSerde {
    CreateAttribute(CreateAttributeStepSerde),
    Debug(String),
    DistortAlongX(Distortion1dStepSerde),
    DistortAlongY(Distortion1dStepSerde),
    Distortion2d(Distortion2dStepSerde),
    GeneratorAdd(GeneratorStepSerde),
    GeneratorSub(GeneratorStepSerde),
    ModifyWithAttribute(ModifyWithAttributeStepSerde),
    TransformAttribute2d(TransformAttribute2dStepSerde),
}

type S = GenerationStepSerde;
type R = GenerationStep;

impl ToStep<GenerationStep> for GenerationStepSerde {
    fn try_convert(self, attributes: &mut Vec<String>) -> Result<GenerationStep> {
        match self {
            S::CreateAttribute(step) => Ok(R::CreateAttribute(step.try_convert(attributes)?)),
            S::Debug(text) => Ok(R::Debug(text)),
            S::DistortAlongX(step) => Ok(R::DistortAlongX(step.try_convert(attributes)?)),
            S::DistortAlongY(step) => Ok(R::DistortAlongY(step.try_convert(attributes)?)),
            S::Distortion2d(step) => Ok(R::Distortion2d(step.try_convert(attributes)?)),
            S::GeneratorAdd(step) => Ok(R::GeneratorAdd(step.try_convert(attributes)?)),
            S::GeneratorSub(step) => Ok(R::GeneratorSub(step.try_convert(attributes)?)),
            S::ModifyWithAttribute(step) => {
                Ok(R::ModifyWithAttribute(step.try_convert(attributes)?))
            }
            S::TransformAttribute2d(step) => {
                Ok(R::TransformAttribute2d(step.try_convert(attributes)?))
            }
        }
    }
}

impl FromStep<GenerationStepSerde> for GenerationStep {
    fn convert(&self, attributes: &mut Vec<String>) -> GenerationStepSerde {
        match self {
            R::CreateAttribute(data) => S::CreateAttribute(data.convert(attributes)),
            R::Debug(text) => S::Debug(text.clone()),
            R::DistortAlongX(data) => S::DistortAlongX(data.convert(attributes)),
            R::DistortAlongY(data) => S::DistortAlongY(data.convert(attributes)),
            R::Distortion2d(data) => S::Distortion2d(data.convert(attributes)),
            R::GeneratorAdd(data) => S::GeneratorAdd(data.convert(attributes)),
            R::GeneratorSub(data) => S::GeneratorSub(data.convert(attributes)),
            R::ModifyWithAttribute(data) => S::ModifyWithAttribute(data.convert(attributes)),
            R::TransformAttribute2d(data) => S::TransformAttribute2d(data.convert(attributes)),
        }
    }
}
