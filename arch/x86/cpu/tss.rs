use super::gdt::{GdtEntry, GdtFlags, ACCESSED, CODE};

#[packed]
pub struct TssEntry {
    prev_tss: u32,
    esp0: u32,
    ss0: u32,
    esp1: u32,
    ss1: u32,
    esp2: u32,
    ss2: u32,
    cr3: u32,
    eip: u32,
    eflags: u32,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    es: u32,
    cs: u32,
    ss: u32,
    ds: u32,
    fs: u32,
    gs: u32,
    ldt: u32,
    trap: u16,
    iomap_base: u16
}

impl TssEntry {
    pub fn gdt_entry(&mut self) -> GdtEntry {
        GdtEntry::seg(self as *mut TssEntry, CODE | ACCESSED, GdtFlags::zero())
    }
}
