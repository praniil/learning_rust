use smart_pointers::drop_traits;
use smart_pointers::ref_counter_smart_pointer;
use smart_pointers::smart_pointer_box;
use smart_pointers::deref_traits;

fn main() {
    // println!("Hello, world!");
    smart_pointer_box();
    deref_traits();
    drop_traits();
    ref_counter_smart_pointer();
}
