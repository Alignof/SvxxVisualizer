mod mmu;

/// Target isa.
#[derive(Copy, Clone)]
pub enum Isa {
    /// 32 bit architecture.
    Rv32,
    /// 64 bit architecture.
    Rv64,
}

/// Type of Virtual-Memory Systems.
pub enum AddrTransMode {
    /// No address transration
    Bare,
    /// Page-Based 32-bit Virtual-Memory Systems
    Sv32,
    /// Page-Based 39-bit Virtual-Memory Systems
    Sv39,
}

/// Privileged level.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

/// Alignment size.
pub enum TransAlign {
    Size8 = 1,
    Size16 = 2,
    Size32 = 4,
    Size64 = 8,
}

/// Usage of transrating address
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransFor {
    /// Fetch an instruction.
    Fetch,
    /// Load a data from memory.
    Load,
    /// For store or AMO instruction.
    StoreAMO,
    /// Delegating fetch.
    Deleg,
}

/// Cause of trap
#[derive(Copy, Clone, Debug)]
#[allow(clippy::enum_clike_unportable_variant)]
pub enum TrapCause {
    InstAddrMisaligned = 0,
    InstAccessFault = 1,
    IllegalInst = 2,
    Breakpoint = 3,
    LoadAddrMisaligned = 4,
    LoadAccessFault = 5,
    StoreAMOAddrMisaligned = 6,
    StoreAMOAccessFault = 7,
    UmodeEcall = 8,
    SmodeEcall = 9,
    MmodeEcall = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
    StoreAMOPageFault = 15,
    SupervisorSoftwareInterrupt = (1 << 31) + 1,
    MachineSoftwareInterrupt = (1 << 31) + 3,
    SupervisorTimerInterrupt = (1 << 31) + 5,
    MachineTimerInterrupt = (1 << 31) + 7,
    SupervisorExternalInterrupt = (1 << 31) + 9,
    MachineExternalInterrupt = (1 << 31) + 11,
}
fn main() {
    println!("Hello, world!");
}
