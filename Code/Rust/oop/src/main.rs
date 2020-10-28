pub struct Foo {
    pub name: String,
    age: u8,
    gender: Gender,
}

impl Foo {
    pub fn get_age(&self) -> u8 {
        self.age // age is returned
    }

    pub fn have_birthday(&mut self) {
        self.age += 1;
    }

    fn have_burial(self) {
        unimplemented!();
    } // `self` will be dropped here
}

fn main() {
    let mut bar = Foo {
        name: String::from("bar"),
        age: 1,
        gender: Gender::Male,
    };

    bar.have_birthday();

    // Party was crazy
    bar.have_burial();

    println!("Bar was {} years old", bar.get_age()); // borrow of moved value: 'bar'
}

// Second implementation block
impl Foo {
    /// Construct a new `Foo` with a given name
    pub fn be_born(name: String) -> Self {
        Foo {
            name,
            age: 0,
            gender: Gender:Female,
        }
    }
}

enum Gender {
    Male,
    Female,
    Unknown,
}
