use core::arch::x86_64::__cpuid;
use crate::println;

pub(crate) struct CpuId {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl CpuId {
    pub unsafe fn new() -> CpuId {
        let cpu = __cpuid(0);
        println!("[CPUID] Registries values: {:?}", cpu);
        let eax = cpu.eax;
        let ebx = cpu.ebx;
        let ecx = cpu.ecx;
        let edx = cpu.edx;

        CpuId { eax, ebx, ecx, edx }
    }

    pub fn print_cpu_vendor<'a>(&self) -> [char; 12] {
        [
            self.ebx as u8 as char, (self.ebx >> 8) as u8 as char,
            (self.ebx >> 16) as u8 as char, (self.ebx >> 24) as u8 as char,
            self.edx as u8 as char, (self.edx >> 8) as u8 as char,
            (self.edx >> 16) as u8 as char, (self.edx >> 24) as u8 as char,
            self.ecx as u8 as char, (self.ecx >> 8) as u8 as char,
            (self.ecx >> 16) as u8 as char, (self.ecx >> 24) as u8 as char,
        ]
    }
}