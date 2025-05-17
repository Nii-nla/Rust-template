/* A marco polo game
If the name Marco is given, the program will respond with Polo.
Otherwise, it will respond with "What's your name?".
*/

pub fn marco_polo(name: &str) -> &str {
    if name == "Marco" {
        "Polo"
    } else {
        "What's your name?"
    }
}
