use crate::inlet::passive::FloatInlet;
use crate::inlet::InletSignal;
use crate::obj::AsObject;
use crate::outlet::{Outlet, OutletSend, OutletSignal, OutletType};
use std::ops::Deref;
use std::rc::Rc;

pub type RcInletSignal = Rc<dyn InletSignal>;
pub type RcOutletSignal = Rc<dyn OutletSignal>;
pub type BoxFloatInlet<T> = Box<Fn(&mut T, puredata_sys::t_float)>;

pub type IntoBuiltControl<T> = (Vec<BoxFloatInlet<T>>);
pub type IntoBuiltGenerator<T> = (Vec<BoxFloatInlet<T>>, Vec<RcOutletSignal>);
pub type IntoBuiltProcessor<T> = (
    Vec<BoxFloatInlet<T>>,
    Vec<RcOutletSignal>,
    Vec<RcInletSignal>,
);

pub trait ControlExternalBuilder<T> {
    fn new_passive_float_inlet(
        &mut self,
        initial_value: puredata_sys::t_float,
    ) -> Rc<dyn Deref<Target = puredata_sys::t_float>>;
    fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>);
    fn new_message_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend>;
}

pub trait SignalGeneratorExternalBuilder<T>: ControlExternalBuilder<T> {
    fn new_signal_outlet(&mut self);
}

pub trait SignalProcessorExternalBuilder<T>: SignalGeneratorExternalBuilder<T> {
    fn new_signal_inlet(&mut self);
}

pub struct Builder<'a, T> {
    obj: &'a mut dyn AsObject,
    dsp_inputs: Vec<Rc<dyn InletSignal>>,
    dsp_outputs: Vec<Rc<dyn OutletSignal>>,
    float_inlets: Vec<Box<Fn(&mut T, puredata_sys::t_float)>>,
}

impl<'a, T> Builder<'a, T> {
    pub fn new(obj: &'a mut dyn AsObject) -> Self {
        Self {
            obj,
            dsp_inputs: Vec::new(),
            dsp_outputs: Vec::new(),
            float_inlets: Vec::new(),
        }
    }

    pub fn dsp_inputs(&self) -> usize {
        self.dsp_inputs.len()
    }

    pub fn dsp_outputs(&self) -> usize {
        self.dsp_outputs.len()
    }
}

impl<'a, T> Into<IntoBuiltControl<T>> for Builder<'a, T> {
    fn into(self) -> IntoBuiltControl<T> {
        (self.float_inlets)
    }
}

impl<'a, T> Into<IntoBuiltGenerator<T>> for Builder<'a, T> {
    fn into(self) -> IntoBuiltGenerator<T> {
        (self.float_inlets, self.dsp_outputs)
    }
}

impl<'a, T> Into<IntoBuiltProcessor<T>> for Builder<'a, T> {
    fn into(self) -> IntoBuiltProcessor<T> {
        (self.float_inlets, self.dsp_outputs, self.dsp_inputs)
    }
}

impl<'a, T> ControlExternalBuilder<T> for Builder<'a, T> {
    fn new_passive_float_inlet(
        &mut self,
        initial_value: puredata_sys::t_float,
    ) -> Rc<dyn Deref<Target = puredata_sys::t_float>> {
        Rc::new(FloatInlet::new(self.obj, initial_value))
    }

    fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>) {
        self.float_inlets.push(func);
    }

    fn new_message_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend> {
        Rc::new(Outlet::new(t, self.obj))
    }
}

impl<'a, T> SignalGeneratorExternalBuilder<T> for Builder<'a, T> {
    fn new_signal_outlet(&mut self) {
        unimplemented!();
    }
}

impl<'a, T> SignalProcessorExternalBuilder<T> for Builder<'a, T> {
    fn new_signal_inlet(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::boxed::Box;

    pub struct A;
    pub struct TestBuilder<T>
    where
        T: Sized,
    {
        float_inlets: Vec<Box<Fn(&mut T, puredata_sys::t_float)>>,
    }

    impl A {
        pub fn print_float(&mut self, f: puredata_sys::t_float) {
            println!("{}", f);
        }
    }

    impl<T> TestBuilder<T>
    where
        T: Sized,
    {
        pub fn call_floats<'a>(&mut self, a: &'a mut T, f: puredata_sys::t_float) {
            for cb in &self.float_inlets {
                (cb)(a, f);
            }
        }

        pub fn new() -> Self {
            Self {
                float_inlets: Vec::new(),
            }
        }
    }

    impl<T> ExternalBuilder<T> for TestBuilder<T> {
        fn new_passive_float_inlet(
            &mut self,
            initial_value: puredata_sys::t_float,
        ) -> Rc<dyn Deref<Target = puredata_sys::t_float>> {
            Rc::new(Box::new(initial_value))
        }

        fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>) {
            self.float_inlets.push(func);
        }

        fn new_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend> {
            Rc::new(())
        }
    }

    #[test]
    fn can_build() {
        let mut a = A;
        let mut builder = TestBuilder::new();

        builder.new_float_inlet(Box::new(|a: &mut A, f: puredata_sys::t_float| {
            a.print_float(f);
        }));
        builder.call_floats(&mut a, 4f32);
    }
}
