use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


#[fe_o_godot_macros::wrap_trait]
pub trait MyTrait {
    fn foo(&self);
}

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct TestNode {

    #[export]
    owner: Option<Gd<Node2D>>,

    base: Base<Node>
}


#[godot_api]
impl TestNode { }

#[fe_o_godot_macros::godot_virtual_dispatch]
impl MyTrait for TestNode {
    fn foo(&self) {
        godot_print!("[TestNode] called 'foo'");
    }
}
