use crate::builder::{
    ExternalBuilder, SignalGeneratorExternalBuilder, SignalProcessorExternalBuilder,
};

pub trait ControlExternal {
    fn new(builder: &mut dyn ExternalBuilder<Self>) -> Self;
}

pub trait SignalGeneratorExternal {
    fn new(builder: &mut dyn SignalGeneratorExternalBuilder<Self>) -> Self;
    fn generate(&mut self, frames: usize, outputs: &[&mut [puredata_sys::t_float]]);
}

//has 1 default signal inlet
pub trait SignalProcessorExternal {
    fn new(builder: &mut dyn SignalProcessorExternalBuilder<Self>) -> Self;
    fn process(
        &mut self,
        frames: usize,
        inputs: &[&[puredata_sys::t_float]],
        outputs: &[&mut [puredata_sys::t_float]],
    );
}
