use ic_stable_structures::{memory_manager::VirtualMemory, DefaultMemoryImpl};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
