use std::fmt::format;


trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("Oh you're a cutie Papagaio! What's your name? {}", self.talk());
    }
}

#[derive(Debug)]
struct Papagaio {
    name: String,
    age: u8,
}

impl Pet for Papagaio {
    fn talk(&self) -> String {
        format!("\nPapagaio! Papagaio! Papagaio! My name is {}!", self.name)
    }
}

fn main() {

    let loro = Papagaio {
        name: "Loro".to_string(),
        age: 2
    };

    loro.greet();

}