mod mmu;

/// Type of Virtual-Memory Systems.
pub enum AddrTransMode {
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

fn main() {
    println!("Hello, world!");
}
