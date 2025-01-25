pub type Eflags = u32;
pub mod eflags {
    use super::Eflags;
    const fn bit(n: usize) -> Eflags { 1 << n }

    pub const CF: Eflags = bit(0);
    pub const PF: Eflags = bit(2);
    pub const AF: Eflags = bit(4);
    pub const ZF: Eflags = bit(6);
    pub const SF: Eflags = bit(7);
    pub const TF: Eflags = bit(8);
    pub const IF: Eflags = bit(9);
    pub const DF: Eflags = bit(10);
    pub const OF: Eflags = bit(11);
    pub const IOPL: Eflags = 0b11 << 12;
    pub const NT: Eflags = bit(14);
    pub const RF: Eflags = bit(16);
    pub const VM: Eflags = bit(17);
    pub const AC: Eflags = bit(18);
    pub const VIF: Eflags = bit(19);
    pub const VIP: Eflags = bit(20);
    pub const CPUID: Eflags = bit(21);
}
