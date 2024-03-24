use metro_engine::prelude::*;

struct Apple;
struct Person {
    name: String,
}
struct Have;

#[derive(EntityEnum)]
enum Entity {
    Apple(Apple),
    Person(Person),
    Have(Have),
    Label(String),
}

fn main() {
    let entity: Entity = "aaa".to_string().into_enum();
}
