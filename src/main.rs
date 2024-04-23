
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let mut v: Vec<String> = Vec::new();

    v.push(String::from("value"));

    match v.get(0) {
        None => (),
        Some(val) => println!("{}", val)
    }
    
    let s = something();
    println!("{}", s);


    let dani = Baby {
        name: "Dani".to_string(),
        is_male: true
    };

    println!("{}", dani.cries());

}


fn something() -> String {
    "somethings".to_string()
}

struct Baby {
    name: String,
    is_male: bool,
}

impl Baby {
    fn cries(&self)-> String {
        let parts1 = "Don't cry, ";
        let parts2 = self.name.as_str();
        let parts3 = " Waaaaa....";
        [parts1, parts2, parts3].join(" ")

    }
}