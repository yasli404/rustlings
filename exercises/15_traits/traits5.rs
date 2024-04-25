// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.


pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// Utilisation de la borne de trait pour permettre le passage de n'importe quel type implémentant les traits SomeTrait et OtherTrait
fn some_func(item: &(impl SomeTrait + OtherTrait)) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(&SomeStruct {});
    some_func(&OtherStruct {});
}