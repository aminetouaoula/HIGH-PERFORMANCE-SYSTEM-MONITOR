pub struct RamCollector {}
use crate::metrics::Collector;
impl Collector for RamCollector {
    fn name(&self) -> String {
        String::from("Ram usage")
    }
    fn collect(&mut self, sys: &sysinfo::System) -> f32 {
        if sys.total_memory() as f32 == 0.0 {
            return 0.0;
        }
        (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0
    }
}
