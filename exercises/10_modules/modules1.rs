// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.


mod sausage_factory {
    // Make the functions public
    pub fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // Now it's public
    pub fn make_sausage() {
        // You can still call the private function from within the module
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    // Now you can access the public function from outside the module
    println!("Secret recipe: {}", sausage_factory::get_secret_recipe());
    // Now you can call the public function from outside the module
    sausage_factory::make_sausage();
}
