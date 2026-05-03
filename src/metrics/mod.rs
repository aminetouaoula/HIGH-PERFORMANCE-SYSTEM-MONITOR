pub mod cpu;
pub mod ram;
pub trait Collector {
    fn name(&self) -> String;
    fn collect(&mut self, sys: &sysinfo::System) -> f32;
}
