fn main(){
    struct Person{
        name : String,
        age : u8,
        language : String
    }
    
    let person = Person{
        name : String::from("Anton"),
        age : 25,
        language: String::from("Rust")
    };
    
    print!("Name is : {}\nAge: {}\nLanguage proficiency: {}", person.name, person.age, person.language);
}
