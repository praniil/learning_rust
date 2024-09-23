use practice::move_ownership;
use practice::ownership_stack_heap;
use practice::transfer_ownership;
use practice::references;
use practice::dangling_ref;

fn main() {
    println!("Hello, world!");
    move_ownership();
    ownership_stack_heap();
    transfer_ownership();
    references();
    dangling_ref();
}
