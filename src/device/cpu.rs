use core::arch::x86_64::__cpuid;
use core::str::from_utf8_unchecked;

unsafe fn get_processor_vendor() -> [u8; 12] {
    let res = __cpuid(0);

    let mut vendor = [0u8; 12];

    vendor[0] = (res.ebx & 0xFF) as u8;
    vendor[1] = ((res.ebx >> 8) & 0xFF) as u8;
    vendor[2] = ((res.ebx >> 16) & 0xFF) as u8;
    vendor[3] = ((res.ebx >> 24) & 0xFF) as u8;
    vendor[4] = (res.edx & 0xFF) as u8;
    vendor[5] = ((res.edx >> 8) & 0xFF) as u8;
    vendor[6] = ((res.edx >> 16) & 0xFF) as u8;
    vendor[7] = ((res.edx >> 24) & 0xFF) as u8;
    vendor[8] = (res.ecx & 0xFF) as u8;
    vendor[9] = ((res.ecx >> 8) & 0xFF) as u8;
    vendor[10] = ((res.ecx >> 16) & 0xFF) as u8;
    vendor[11] = ((res.ecx >> 24) & 0xFF) as u8;

    vendor
}

unsafe fn get_processor_name() -> [u8; 48] { // 48 байт = 3 части по 16 байт
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

pub unsafe fn get_processor_name_str() -> &'static str {
    static mut CPU_NAME: [u8; 48] = [0; 48];

    let cpu_name = get_processor_name();

    let len = cpu_name.len().min(CPU_NAME.len());

    CPU_NAME[..len].copy_from_slice(&cpu_name[..len]);

    let trimmed_len = CPU_NAME.iter().position(|&b| b == 0).unwrap_or(CPU_NAME.len());

    from_utf8_unchecked(&CPU_NAME[..trimmed_len])
}

pub unsafe fn get_processor_vendor_str() -> &'static str {
    static mut VENDOR: [u8; 12] = [0; 12];

    let vendor = get_processor_vendor();

    for (i, byte) in vendor.iter().enumerate() {
        VENDOR[i] = *byte;
    }

    from_utf8_unchecked(&VENDOR)
}