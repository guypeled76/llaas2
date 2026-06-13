mod models;
pub mod messages;

fn main() {
    models::translate::apply();
    models::pos::apply();
    models::sentiment::apply();
    models::keywords::apply();
    models::classification::apply();
}











