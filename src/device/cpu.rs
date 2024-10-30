use core::arch::x86_64::{CpuidResult, __cpuid, __cpuid_count};
use crate::println;

pub unsafe fn print_cpu_vendor<'a>() -> [char; 12] {
    let res = __cpuid(0);

    [
        res.ebx as u8 as char, (res.ebx >> 8) as u8 as char,
        (res.ebx >> 16) as u8 as char, (res.ebx >> 24) as u8 as char,
        res.edx as u8 as char, (res.edx >> 8) as u8 as char,
        (res.edx >> 16) as u8 as char, (res.edx >> 24) as u8 as char,
        res.ecx as u8 as char, (res.ecx >> 8) as u8 as char,
        (res.ecx >> 16) as u8 as char, (res.ecx >> 24) as u8 as char,
    ]
}

pub unsafe fn get_processor_name() -> [u8; 48] { // 48 байт = 3 части по 16 байт
    let mut name = [0u8; 48];

    for (i, leaf) in (0x80000002..=0x80000004).enumerate() {
        let result = __cpuid(leaf);
        let offset = i * 16;

        name[offset..offset + 16].copy_from_slice(&[
            (result.eax & 0xFF) as u8,
            ((result.eax >> 8) & 0xFF) as u8,
            ((result.eax >> 16) & 0xFF) as u8,
            ((result.eax >> 24) & 0xFF) as u8,
            (result.ebx & 0xFF) as u8,
            ((result.ebx >> 8) & 0xFF) as u8,
            ((result.ebx >> 16) & 0xFF) as u8,
            ((result.ebx >> 24) & 0xFF) as u8,
            (result.ecx & 0xFF) as u8,
            ((result.ecx >> 8) & 0xFF) as u8,
            ((result.ecx >> 16) & 0xFF) as u8,
            ((result.ecx >> 24) & 0xFF) as u8,
            (result.edx & 0xFF) as u8,
            ((result.edx >> 8) & 0xFF) as u8,
            ((result.edx >> 16) & 0xFF) as u8,
            ((result.edx >> 24) & 0xFF) as u8,
        ]);
    }

    name
}