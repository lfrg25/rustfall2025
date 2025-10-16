enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        for fruits in &self.fruit{
            match fruits{
                Fruit::Apple(_) => println!("Apple"),
                Fruit::Banana(_) => println!("Banana"),
                Fruit::Tomato(_) => println!("Tomato"),
                }
        }
    }

    fn tell_me_joke(fruit: &Fruit) { 
        match fruit{
            Fruit::Apple(applejoke) => println!("Apple Joke: {}", applejoke),
            Fruit::Banana(bananajoke) => println!("Banana Joke: {}", bananajoke),
            Fruit::Tomato(tomatojoke) => println!("Tomato Joke: {}", tomatojoke),
        }

    }
}

fn main(){
    let a = "What is an apples favorite computer? A macintosh".to_string();
    let b = "Why don’t bananas ever get lonely? Because they all hang out in bunches!".to_string();
    let t = "Why is a tomato always rushing? Because hes trying to ketchup".to_string();
    let fruits = vec![Fruit::Banana(b),Fruit::Apple(a),Fruit::Tomato(t)];
    let grocery_store = Inventory {
        fruit:fruits,
    };

    grocery_store.available_fruits();

for f in &grocery_store.fruit {
    Inventory::tell_me_joke(f);
}

   
}
