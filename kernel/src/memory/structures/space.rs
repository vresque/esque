/// # Memory Space
/// This structure represents a *virtual* address space.
/// This space contains a reference to the page table
pub struct Space {
    cr3: Frame,
}
