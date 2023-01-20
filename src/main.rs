use std::env;

fn main() {
    // |- Get propositions
    let mut arguments = env::args().collect::<Vec<String>>();
    // Removes the first element of arguments, the path of this executable.
    arguments.remove(0);
    // Get propositions -|
}
