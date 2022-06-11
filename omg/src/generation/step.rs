use crate::data::map::Map2d;
use crate::generation::attributes::create::CreateAttributeStep;
use crate::generation::attributes::distortion1d::Distortion1dStep;
use crate::generation::attributes::distortion2d::Distortion2dStep;
use crate::generation::attributes::generator::GeneratorStep;
use crate::generation::attributes::modify::ModifyWithAttributeStep;
use crate::generation::attributes::transformer::TransformAttribute2dStep;
use GenerationStep::*;

/// A step during [`MapGeneration`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GenerationStep {
    CreateAttribute(CreateAttributeStep),
    Debug(String),
    DistortAlongX(Distortion1dStep),
    DistortAlongY(Distortion1dStep),
    Distortion2d(Distortion2dStep),
    GeneratorAdd(GeneratorStep),
    GeneratorSub(GeneratorStep),
    ModifyWithAttribute(ModifyWithAttributeStep),
    TransformAttribute2d(TransformAttribute2dStep),
}

impl GenerationStep {
    pub fn debug<S: Into<String>>(text: S) -> GenerationStep {
        Debug(text.into())
    }
    /// Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        match self {
            CreateAttribute(step) => step.run(map),
            Debug(text) => info!("Debug step: {}", text),
            DistortAlongX(step) => step.distort_along_x(map),
            DistortAlongY(step) => step.distort_along_y(map),
            Distortion2d(step) => step.run(map),
            GeneratorAdd(step) => step.add(map),
            GeneratorSub(step) => step.sub(map),
            ModifyWithAttribute(step) => step.run(map),
            TransformAttribute2d(step) => step.run(map),
        }
    }
}
