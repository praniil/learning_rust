use rust_lifetimes::dangling_ref;
use rust_lifetimes::generical_lifetime_annotation;
use rust_lifetimes::generics_traits_lifetime;
use rust_lifetimes::lifetime_annoation_struct;
use rust_lifetimes::lifetime_elision;
fn main() {
    dangling_ref();
    generical_lifetime_annotation();
    lifetime_annoation_struct();
    lifetime_elision();
    generics_traits_lifetime();
}
