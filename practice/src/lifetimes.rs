fn largest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}


struct Animal<'a>{
    speak: &'a str,
    animal_type: &'a str,
}

impl<'a> Animal<'a> {
    fn speak(&self) -> &str {
        &self.speak
    }
}

pub fn lifetime() {
    let string1 = String::from("Helloaa");
    let largest1;
    {
        let string2 = String::from("World");
        largest1 = largest(string1.as_str(), string2.as_str());
        println!("{largest1}");
    }
    let cat = Animal{
        speak: "meow",
        animal_type: "domestic",
    };

    {
        let speak = cat.speak();
        println!("{speak}")
    }
}