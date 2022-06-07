use crate::data::map::Map2d;
use crate::generation::attributes::create::CreateAttribute;
use crate::generation::attributes::distortion1d::Distortion1d;
use crate::generation::attributes::distortion2d::Distortion2d;
use crate::generation::attributes::generator::GeneratorStep;
use crate::generation::attributes::modify::ModifyWithAttribute;
use crate::generation::attributes::transformer::TransformAttribute2d;
use GenerationStep::*;

/// A step during [`MapGeneration`].
pub enum GenerationStep {
    CreateAttribute(CreateAttribute),
    DistortAlongX(Distortion1d),
    DistortAlongY(Distortion1d),
    Distortion2d(Distortion2d),
    GeneratorAdd(GeneratorStep),
    GeneratorSub(GeneratorStep),
    ModifyWithAttribute(ModifyWithAttribute),
    TransformAttribute2d(TransformAttribute2d),
}

impl GenerationStep {
    /// Runs the step.
    pub fn run(&self, map: &mut Map2d) {
        match self {
            CreateAttribute(step) => step.run(map),
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
