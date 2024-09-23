mod ownership;
mod ownership_function;
mod transfering_ownership;
mod mut_imut_ref;
mod dangling_ref;

pub use ownership::move_ownership;
pub use ownership_function::ownership_stack_heap;
pub use transfering_ownership::transfer_ownership;
pub use mut_imut_ref::references;
pub use dangling_ref::dangling_ref;