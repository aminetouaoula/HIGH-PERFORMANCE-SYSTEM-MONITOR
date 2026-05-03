pub struct CpuCollector {}
use crate::metrics::Collector;
impl Collector for CpuCollector {
    fn name(&self) -> String {
        String::from("CPU usage")
    }
    fn collect(&mut self, sys: &sysinfo::System) -> f32 {
        sys.global_cpu_info().cpu_usage()
    }
}
